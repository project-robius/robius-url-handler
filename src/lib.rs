mod sys;

pub fn register_handler(handler: fn(&str)) {
    sys::register_handler(handler)
}
