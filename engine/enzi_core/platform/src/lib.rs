pub mod platform_linux;
pub mod errors;

#[cfg(target_os = "linux")]
pub mod platform_api {
    pub use super::platform_linux::*;
}

#[cfg(target_os = "windows")]
pub mod platform_api {
    pub use super::platform_windows::*;
}

//pub mod platform_win32;