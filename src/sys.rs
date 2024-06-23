cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub(crate) use linux::register_handler;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub(crate) use macos::register_handler;
    } else if #[cfg(target_family = "windows")] {
        mod windows;
        pub(crate) use windows::register_handler;
    } else {
        mod unsupported;
        pub(crate) use unsupported::register_handler;
    }
}
