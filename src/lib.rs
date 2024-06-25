use std::env::Args;

mod sys;

pub fn register_handler(handler: fn(&str), parser: Option<fn(Args) -> Option<String>>) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "linux", target_os = "windows"))] {
            let parser = parser.unwrap_or(default_parser);
            if let Some(parsed_url) = parser(std::env::args()) {
                handler(&parsed_url);
            }
        } else {
            sys::register_handler(handler)
        }
    }
}

fn default_parser(mut args: Args) -> Option<String> {
    args.nth(1)
}
