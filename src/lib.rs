use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use wasm_bindgen::prelude::*;
use web_sys::{wasm_bindgen::JsCast, Element, HtmlButtonElement, HtmlTextAreaElement};

#[cfg(feature = "default-form")]
mod default_form;
#[cfg(feature = "default-form")]
pub use default_form::set_default_hook_with;

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

/// The ID of the text area which is loaded with the stack trace in the default form.
pub const FORM_TEXTAREA_ID: &str = "panic_info_form_text";

/// The ID of the `Send Report` button element in the default form.
pub const FORM_SUBMIT_ID: &str = "panic_info_form_submit";

/// Information about the panic that occurred, potentially useful to report.
///
/// Why not [`PanicInfo`](std::panic::PanicInfo)? `PanicInfo` is unable to be owned, doesn't
/// implement `Clone`, and is `!Send`, making it unable to pass to a callback.
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

/// Set the panic hook.
///
/// # Params
/// `container_id`: The ID of the HTML element that will be unmounted in favor of the form.\
/// `form_html`: The raw HTML that will replace the container.\
/// `on_panic_callback`: A closure that is triggered on panic automatically.\
/// `submit_callback`: The closure that will run when the user hits the send report button.
///
/// # Panics
/// This will panic (ironically) if the panic is caught in a headless environment.
pub fn set_hook_with<F1, F2>(
    container_id: impl Into<String>,
    form_html: impl Into<String>,
    on_panic_callback: F1,
    submit_callback: F2,
) where
    F1: Fn(&WasmPanicInfo) + Send + Sync + 'static,
    F2: Fn(&WasmPanicInfo) + Send + Sync + 'static,
{
    let form_html = form_html.into();
    let container_id = container_id.into();
    let already_hydrated: AtomicBool = AtomicBool::new(false);
    let on_panic_callback = Arc::new(on_panic_callback);
    let submit_callback = Arc::new(submit_callback);

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
        // Print error
        error(console_message.clone());

        if already_hydrated.swap(true, Ordering::Relaxed) {
            // Error already hydrated to the form body
            return;
        }

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
        // Trigger callback on panic.
        on_panic_callback(&wasm_panic_info);

        // Retrieve the parent container we will be replacing with the report form.
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let parent: Element = document
            .get_element_by_id(&container_id)
            .unwrap_or_else(|| {
                panic!("`web_panic_report`: element `{container_id}` does not exist in the DOM",)
            });
        // Replace inner html with our report form
        parent.set_inner_html(&form_html);

        // Hydrate the stack trace
        let text_area: HtmlTextAreaElement = document
            .get_element_by_id(FORM_TEXTAREA_ID)
            .expect("form text id doesn't exist")
            .unchecked_into();
        text_area.set_value(&console_message);

        // Add onclick handler to the submit button
        let submit_button: HtmlButtonElement = document
            .get_element_by_id(FORM_SUBMIT_ID)
            .expect("form submit id doesn't exist")
            .unchecked_into();
        let closure: Closure<dyn Fn()> = Closure::new({
            let callback = submit_callback.clone();
            move || {
                callback(&wasm_panic_info);
            }
        });
        let closure = closure.into_js_value();
        submit_button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    }));
}
