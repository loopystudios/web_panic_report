use std::panic;

use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use wasm_bindgen::prelude::*;

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

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        // Not mandatory
        console_error_panic_hook::hook(panic_info);

        let mut report_msg = panic_info.to_string();
        report_msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        report_msg.push_str(&stack);

        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        //let body = document.body().expect("document expect to have have a body");
        let _ = document
            .get_element_by_id("panic_info_form_container")
            .expect("should have #panic_info_form_container on the page")
            .dyn_ref::<HtmlElement>()
            .expect("#loading should be an `HtmlElement`")
            .style()
            .set_property("display", "block");
        document
            .get_element_by_id("panic_info_form_text")
            .expect("should have #panic_info_form_text on the page")
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("#loading should be an `HtmlElement`")
            .set_inner_text(&report_msg);
    }));
    some_nested_function();
}

fn some_nested_function() {
    panic!("Force panic to see how a report looks like.");
}
