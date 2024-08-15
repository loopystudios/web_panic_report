# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

## Unreleased

This release has an [MSRV][] of 1.75.

## [0.3.0] - 2024/08/15

This release has an [MSRV][] of 1.75.

### Added

- `set_hook_with` now takes an additional argument, `on_panic_callback` which is triggered automatically on panic. ([#3] by [@simbleau])

### Fixed

- If a panic occurs after the first panic (which can happen in multi-threaded apps, e.g. Bevy games), it will only be logged to stdout. Hence, only the first panic that occurred is preserved. ([#3] by [@simbleau])

## [0.2.0] - 2024/04/15

This release has an [MSRV][] of 1.75.

### Added

- `FORM_TEXTAREA_ID` is now public, which has the element ID of the stack trace text area.
- `FORM_SUBMIT_ID` is now public, which has the element ID of the submit button.

### Changed

- `set_hook_with` now takes the additional argument of an HTML form structure. The previous, default form can still be used with `web_panic_report::set_default_hook_with`, however requires the new (default) cargo feature: `default-form`. ([#2] by [@simbleau])

## [0.1.1] - 2024/03/17

This release has an [MSRV][] of 1.75.

### Fixed

- Any patch version of `web-sys` or `wasm-bindgen` can now be used.

## [0.1.0] - 2024/03/17

This release has an [MSRV][] of 1.75.

- Initialize release ([@Vrixyz] and [@simbleau])

[@simbleau]: https://github.com/simbleau
[@Vrixyz]: https://github.com/Vrixyz

[#2]: https://github.com/loopystudios/web_panic_report/pull/2
[#3]: https://github.com/loopystudios/web_panic_report/pull/3

[Unreleased]: https://github.com/loopystudios/web_panic_report/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/loopystudios/web_panic_report/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/loopystudios/web_panic_report/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/loopystudios/web_panic_report/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/loopystudios/web_panic_report/releases/tag/v0.1.0

[MSRV]: README.md#minimum-supported-rust-version-msrv
