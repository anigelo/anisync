cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use unix::copy_file;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::copy_file;
    }
}