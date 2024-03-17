use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement, HtmlTextAreaElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

/// The ID of the element in `form.html` which receives the stack trace
const FORM_TEXTAREA_ID: &str = "panic_info_form_text";

/// The ID of the element in `form.html` which receives the stack trace
const FORM_SUBMIT_ID: &str = "panic_info_form_submit";

/// Information about the panic that occurred, potentially useful to report.
///
/// Why not [`PanicInfo`](std::panic::PanicInfo)? `PanicInfo` is unable to be owned, doesn't implement `Clone`, and is `!Send`, making it unable to pass to a callback.
#[derive(Debug)]
pub struct WasmPanicInfo {
    /// The file that triggered the panic
    pub file: String,
    /// The line the panic was triggered on
    pub line: u32,
    /// The column the panic was triggered on
    pub col: u32,
    /// Equivalent to [`PanicInfo`](std::panic::PanicInfo)'s `to_string()` method.
    pub display: String,
    /// The full stack trace retrieved.
    pub stack: String,
}
impl std::fmt::Display for WasmPanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display.fmt(f)
    }
}

/// Set the panic hook
pub fn set_hook_with<F>(container_id: impl Into<String>, submit_callback: F)
where
    F: Fn(&WasmPanicInfo) + Send + Sync + 'static,
{
    let container_id = container_id.into();
    let callback = Arc::new(submit_callback);

    std::panic::set_hook(Box::new(move |panic_info| {
        // Collect stack trace
        let e = Error::new();
        let stack = e.stack();

        // Log to stderr on console
        let console_message = {
            let mut stack_trace = panic_info.to_string();
            stack_trace.push_str("\n\nStack:\n\n");
            stack_trace.push_str(&stack);
            stack_trace.push_str("\n\n");
            stack_trace
        };
        error(console_message.clone());

        // Retrieve the parent container we will be replacing with the report form.
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let parent: Element = document
            .get_element_by_id(&container_id)
            .unwrap_or_else(|| {
                panic!("`web_panic_report`: element `{container_id}` does not exist in the DOM",)
            });

        // Replace inner html with our report form
        parent.set_inner_html(include_str!("form.html"));
        // Replace the stack trace
        let text_area: HtmlTextAreaElement = document
            .get_element_by_id(FORM_TEXTAREA_ID)
            .expect("form text id doesn't exist")
            .unchecked_into();
        text_area.set_value(&console_message);
        // TODO: Add onclick handler to button
        let submit_button: HtmlButtonElement = document
            .get_element_by_id(FORM_SUBMIT_ID)
            .expect("form submit id doesn't exist")
            .unchecked_into();
        // Add onclick handler to the button
        let wasm_panic_info = WasmPanicInfo {
            file: panic_info
                .location()
                .map(|pi: &std::panic::Location<'_>| pi.file())
                .unwrap_or_default()
                .to_owned(),
            line: panic_info
                .location()
                .map(|pi| pi.line())
                .unwrap_or_default(),
            col: panic_info
                .location()
                .map(|pi| pi.column())
                .unwrap_or_default(),
            display: panic_info.to_string(),
            stack: e.stack(),
        };
        let closure: Closure<dyn Fn()> = Closure::new({
            let callback = callback.clone();
            move || {
                callback(&wasm_panic_info);
            }
        });
        let closure = closure.into_js_value();
        submit_button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    }));
}
