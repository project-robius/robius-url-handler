// TODO: This relies on details in the Moxin .desktop file.

pub(crate) fn register_handler(handler: fn(&str)) {
    if let Some(url) = std::env::args().nth(1) {
        handler(&url);
    }
}
