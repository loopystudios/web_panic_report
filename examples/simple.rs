use wasm_bindgen::prelude::*;

/// Nothing special here, we just want to call the Window `alert` function in our custom callback.
/// This bindgen gives us access to that.
#[wasm_bindgen]
extern "C" {
    /// The Window `alert` function.
    fn alert(s: &str);
}

/// This is called from a button on the web example to allow the user to trigger a panic.
#[no_mangle]
pub extern "C" fn trigger_panic() {
    panic!("You triggered the panic!");
}

fn main() {
    // Set the panic hook at the beginning of your program
    web_panic_report::set_hook_with("test-container", |panic_info| {
        alert(&format!(
            "This is a custom callback!\n\n{}",
            panic_info.display
        ));
    });
}
