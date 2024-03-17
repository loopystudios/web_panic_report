#[test]
fn can_set_as_hook() {
    web_panic_report::set_hook_with("", |_| {});
}
