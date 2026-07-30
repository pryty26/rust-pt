// macOS
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-macos-arm64-v8a.zip";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-macos-64.zip";

// Windows
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-windows-64.zip";

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-windows-32.zip";

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-windows-arm64-v8a.zip";

// Linux
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-linux-64.zip";

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-linux-32.zip";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-linux-arm64-v8a.zip";

// Linux ARM 32-bit (v7a and others)
#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-linux-arm32-v7a.zip";

// FreeBSD
#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-freebsd-64.zip";

#[cfg(all(target_os = "freebsd", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-freebsd-32.zip";

// OpenBSD
#[cfg(all(target_os = "openbsd", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-openbsd-64.zip";

#[cfg(all(target_os = "openbsd", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &'static str = "Xray-openbsd-32.zip";

// Unsupported platform fallback
#[cfg(not(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "windows", target_arch = "x86"),
    all(target_os = "windows", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "x86"),
    all(target_os = "linux", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "arm"),
    all(target_os = "freebsd", target_arch = "x86_64"),
    all(target_os = "freebsd", target_arch = "x86"),
    all(target_os = "openbsd", target_arch = "x86_64"),
    all(target_os = "openbsd", target_arch = "x86"),
)))]
compile_error!(
    "Unsupported platform: Xray-core does not provide pre-built binaries for this target"
);
#[cfg(windows)]
pub(crate) const XRAY_CMD: &str = "xray.exe";
#[cfg(not(windows))]
pub(crate) const XRAY_CMD: &str = "xray";
