use crate::WasmPanicInfo;

/// The default form HTML.
const FORM_HTML: &str = include_str!("form.html");

/// Set the panic hook, with a default form.
///
/// # Params
/// `container_id`: The ID of the HTML element that will be unmounted in favor of the form.\
/// `submit_callback`: The closure that will run when the user hits the send report button.
///
/// # Panics
/// This will panic (ironically) if the panic occurs in a headless environment.
pub fn set_default_hook_with<F>(container_id: impl Into<String>, submit_callback: F)
where
    F: Fn(&WasmPanicInfo) + Send + Sync + 'static,
{
    crate::set_hook_with(container_id, FORM_HTML, |_| {}, submit_callback)
}
