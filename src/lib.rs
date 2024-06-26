//! A crate for registering application URL handlers.
//!
//! Note that this only handles the runtime aspect, the compile-time aspect must
//! be done separately e.g. by using `cargo packager`.
//!
//! # Argument parser
//!
//! On Linux and Windows, this crate assumes that the URL is passed through the
//! command-line arguments and hence a parser is necessary to convert the
//! command-line arguments into a URL. The parser should return `None` if the
//! application is not being launched with a URL.
//!
//! The default argument parser used by `register_handler` assumes that the
//! first argument is the URL, and if no argument is passed, then the
//! application is being launched without a URL. This is designed to work with
//! `cargo packager`.
//!
//! For more complicated logic e.g. using a CLI parser with a `--url` flag, the
//! `register_handler_with_parser` function can be used to provide a custom
//! parser.
//!
//! # Singleton application
//!
//! On macOS, if an instance of the application is already open, and a URL with
//! a matching scheme is opened, the handler will be called again with the new
//! URL.
//!
//! However, on Windows and Linux, a new instance of the application will be
//! launched and so it is up to the developer to manage coordination between
//! separate application processes.

// TODO: Could provide example on how to manage coordination between separate
// application processes.

use std::env::Args;

mod sys;

/// Registers a URL handler with the default argument parser.
///
/// The handler is given the URL string (including the scheme).
///
/// See the crate-level documentation for more information on the argument
/// parser.
pub fn register_handler(handler: fn(&str)) {
    fn default_arg_parser(mut args: Args) -> Option<String> {
        args.nth(1)
    }

    register_handler_with_parser(handler, default_arg_parser)
}

/// Registers a URL handler with a custom argument parser.
///
/// The handler is given the URL string (including the scheme).
///
/// See the crate-level documentation for more information on the argument
/// parser.
pub fn register_handler_with_parser<T>(handler: fn(&str), arg_parser: T)
where
    T: FnOnce(Args) -> Option<String>,
{
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "windows"))] {
            if let Some(parsed_url) = arg_parser(std::env::args()) {
                handler(&parsed_url);
            }
        } else {
            drop(arg_parser);
            sys::register_handler(handler)
        }
    }
}
