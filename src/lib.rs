use std::env::Args;

mod sys;

pub fn register_handler(handler: fn(&str), _arg_parser: Option<fn(Args) -> Option<String>>) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "windows"))] {
            fn default_arg_parser(mut args: Args) -> Option<String> {
                args.nth(1)
            }

            let arg_parser = _arg_parser.unwrap_or(default_arg_parser);
            if let Some(parsed_url) = arg_parser(std::env::args()) {
                handler(&parsed_url);
            }
        } else {
            sys::register_handler(handler)
        }
    }
}
