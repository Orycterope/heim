//! OS-specific extensions for public types.

cfg_if::cfg_if! {
    // Any unix except OpenBSD
    if #[cfg(any(doc, all(unix, not(target_os = "openbsd"))))] {
        pub mod unix;
    } else if #[cfg(any(doc, target_os = "windows"))] {
        pub mod windows;
    }
}
