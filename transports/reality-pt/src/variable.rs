// File: transports\reality-pt\src\variable.rs
// Directory: transports\reality-pt\src
// Filename: variable.rs
//======================================================================

#![allow(missing_docs)]

/// macOS (Apple Silicon)
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &str = "Xray-macos-arm64-v8a.zip";

/// macOS (Intel)
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &str = "Xray-macos-64.zip";

// Windows
/// Windows (64-bit)
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &str = "Xray-windows-64.zip";

/// Windows (32-bit)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &str = "Xray-windows-32.zip";

/// Windows (ARM64)
#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &str = "Xray-windows-arm64-v8a.zip";

// Linux
/// Linux (64-bit)
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &str = "Xray-linux-64.zip";

/// Linux (32-bit)
#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &str = "Xray-linux-32.zip";

/// Linux (ARM64)
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub const XRAY_FILE_NAME: &str = "Xray-linux-arm64-v8a.zip";

/// Linux (ARM 32-bit v7a)
#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub const XRAY_FILE_NAME: &str = "Xray-linux-arm32-v7a.zip";

// FreeBSD
/// FreeBSD (64-bit)
#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &str = "Xray-freebsd-64.zip";

/// FreeBSD (32-bit)
#[cfg(all(target_os = "freebsd", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &str = "Xray-freebsd-32.zip";

// OpenBSD
/// OpenBSD (64-bit)
#[cfg(all(target_os = "openbsd", target_arch = "x86_64"))]
pub const XRAY_FILE_NAME: &str = "Xray-openbsd-64.zip";

/// OpenBSD (32-bit)
#[cfg(all(target_os = "openbsd", target_arch = "x86"))]
pub const XRAY_FILE_NAME: &str = "Xray-openbsd-32.zip";

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

/// The command used to execute the Xray binary
#[cfg(windows)]
pub(crate) const XRAY_CMD: &str = "xray.exe";

/// The command used to execute the Xray binary
#[cfg(not(windows))]
pub(crate) const XRAY_CMD: &str = "xray";
