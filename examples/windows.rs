use robius_url_handler::register_handler;
use windows::{
    core::{w, PCWSTR},
    Win32::System::Registry::{RegCreateKeyExW, RegOpenKeyW, HKEY_CURRENT_USER, HKEY_USERS},
};

const SCHEME: PCWSTR = w!("SOFTWARE\\Classes\\moxin");
const VERSION: &str = "b";
const VERSION2: &str = "c";

fn main() {
    std::fs::write("C:\\Users\\Klim\\Desktop\\log1", VERSION).unwrap();

    register_handler(handler);

    std::fs::write("C:\\Users\\Klim\\Desktop\\log1", VERSION).unwrap();

    loop {}
}

fn register_scheme(scheme: PCWSTR) {
    let application_path = std::env::args().next().unwrap();

    let out = std::ptr::null_mut();
    let err = unsafe {
        RegCreateKeyExW(
            HKEY_USERS,
            // scheme,
            w!(""),
            0,
            PCWSTR::null(),
            Default::default(),
            Default::default(),
            None,
            out,
            None,
        )
    };
    // todo!("{err:?} {out:?}");
}

fn handler(url: &str) {
    std::fs::write("C:\\Users\\Klim\\Desktop\\log2", crate::VERSION).unwrap();
}
