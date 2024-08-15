/// This is called from a button on the web example to allow the user to trigger a panic.
#[no_mangle]
pub extern "C" fn trigger_panic() {
    panic!("You triggered the panic!");
}

fn main() {
    let my_form = format!(
        r#"
<div style="display: flex; flex-direction: column; width: 100%; height: 100%; background-color: white; color: red">
    This is a custom panic form! The stack trace is hidden from users.
    <textarea readonly id="{}" style="display: none;"></textarea>
    <br />
    <input id="{}" type="submit" value="See Panic Info" style="width: fit-content;" />
</div>
    "#,
        web_panic_report::FORM_TEXTAREA_ID,
        web_panic_report::FORM_SUBMIT_ID
    );

    // Set the panic hook at the beginning of your program
    web_panic_report::set_hook_with(
        "test-container",
        my_form,
        // No-op on panic
        |_| {},
        // Submit button will cause a web browser alert
        |panic_info| {
            web_sys::window()
                .unwrap()
                .alert_with_message(&panic_info.display.to_string())
                .unwrap();
        },
    );
}
