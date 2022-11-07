cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
    } else if #[cfg(windows)] {
        mod windows;
    }
}
