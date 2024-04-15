/// This is called from a button on the web example to allow the user to trigger a panic.
#[no_mangle]
pub extern "C" fn trigger_panic() {
    panic!("You triggered the panic!");
}

fn main() {
    // Set the panic hook at the beginning of your program
    web_panic_report::set_default_hook_with("test-container", |panic_info| {
        web_sys::window()
            .unwrap()
            .alert_with_message(&format!(
                "This is a custom callback!\n\n{}",
                panic_info.display
            ))
            .unwrap();
    });
}
