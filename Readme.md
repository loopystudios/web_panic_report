# Add reporting to your web panics

The goal of this project is NOT to provide a full-fledge error reporting and analytics,
It's merely to help understanding how one can report such information.

For full-fledged solutions, check https://sentry.io/welcome/, they [share](https://sentry.engineering/blog) good information.

When working on wasm, a popular crate is https://github.com/rustwasm/console_error_panic_hook.
Very useful for debugging, it will not help us with reporting.

This repository heavily draws from https://github.com/rustwasm/console_error_panic_hook,
and explores a strategy to add panic reporting.

Currently, a form is declared in a html file, and the panic hook displays it and feeds it with the panic stacktrace.

## Unknowns

it works fine on debug, but on release it's currently less readable, but there are a few solutions (DWARF, source maps, debug id), see https://blog.sentry.io/the-pain-of-debugging-webassembly/ and https://sentry.engineering/blog/the-case-for-debug-ids
