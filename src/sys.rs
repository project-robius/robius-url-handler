cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos;
        pub(crate) use macos::register_handler;
    } else {
        mod unsupported;
        #[allow(unused_imports)]
        pub(crate) use unsupported::register_handler;
    }
}
