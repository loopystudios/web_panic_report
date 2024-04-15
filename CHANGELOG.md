# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## Unreleased

## 0.2.0

### added

- `FORM_TEXTAREA_ID` is now public, which has the element ID of the stack trace text area.
- `FORM_SUBMIT_ID` is now public, which has the element ID of the submit button.

### changed

- `set_hook_with` now takes the additional argument of an HTML form structure. The previous, default form can still be used with `web_panic_report::set_default_hook_with`, however requires the new (default) cargo feature: `default-form`.

## 0.1.1

### changed

- Any patch version of `web-sys` or `wasm-bindgen` can now be used.

## 0.1.0

- Initialize release
