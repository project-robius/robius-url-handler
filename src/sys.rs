mod unix;
cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos;
        pub(crate) use macos::register_handler;
    } else if #[cfg(target_family = "windows")] {
        mod windows;
        pub(crate) use windows::*;
    } else {
        mod unsupported;
        pub(crate) use unsupported::*;
    }
}
