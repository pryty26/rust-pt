// File: transports\reality-pt\src\download.rs
// Directory: transports\reality-pt\src
// Filename: download.rs
//======================================================================

pub use crate::variable::XRAY_FILE_NAME;
use anyhow::Result;
// use std::env::set_current_dir;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use zip::ZipArchive;
/// unzip the downloaded xray zip file
fn unzip(file: PathBuf) -> Result<PathBuf> {
    let file = File::open(file)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(".")?;
    #[cfg(unix)]
    let exe_path = PathBuf::from("xray");
    #[cfg(windows)]
    let exe_path = PathBuf::from("xray.exe");

    Ok(exe_path)
}
/// Download the xray from the github
///
/// # Errors
/// This function returns an error if:
/// - The `curl` command fails to execute
/// - The downloaded zip file cannot be unzipped
/// - The zip file cannot be removed after extraction
///
/// # Panics
/// This function will panic if:
/// - The `curl` command cannot be spawned (e.g., curl not installed)
/// - The `curl` command exits with a non-zero status code
pub fn get_xray() -> Result<()> {
    match Command::new("curl")
        .arg("-L")
        .arg(format!(
            "https://github.com/XTLS/Xray-core/releases/latest/download/{XRAY_FILE_NAME}"
        ))
        .arg("-o")
        .arg("xray.zip")
        .output()
    {
        Ok(output) => {
            assert!(
                output.status.success(),
                "xray exited with error: {}",
                output.stderr.iter().map(|&c| c as char).collect::<String>()
            );
        },
        Err(e) => panic!("failed to execute xray {e}"),
    }
    let _file = unzip(PathBuf::from("xray.zip"))?;
    fs::remove_file("xray.zip")?;

    // Get and Set permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = fs::set_permissions(&_file, fs::Permissions::from_mode(0o755)) {
            eprintln!("Error: unable to change permissions");
        }
    }
    Ok(())
}
/*
fn start_xray() -> Result<()> {
    set_current_dir(path);
}
*/
