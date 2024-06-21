use robius_url_handler::register_handler;

fn main() {
    register_handler(handler);
    loop {}
}

fn handler(url: &str) {
    println!("called handler with url: {url}");
}
