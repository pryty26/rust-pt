use anyhow::Result;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
pub mod variable;
use variable::XRAY_CMD;
pub mod download;
/// Creates a file at the specified path,
/// creating any necessary parent directories.
pub fn create_all_path(path: &PathBuf) -> Result<File> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(File::create(path)?)
}
/// Creates a pair of x25519 keys and writes them to the specified files.
pub fn create_keys(
    private_key_path: Option<&PathBuf>,
    public_key_path: Option<&PathBuf>,
) -> Result<()> {
    let output = match Command::new(XRAY_CMD).arg("x25519").output() {
        Ok(output) => {
            if !output.status.success() {
                panic!(
                    "xray exited with error: {}",
                    output.stderr.iter().map(|&c| c as char).collect::<String>()
                );
            }
            output
        }
        Err(_) => panic!("failed to execute xray"),
    };
    let output_str = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = output_str.trim().split('\n').collect();

    let private_key = lines[0].split(": ").nth(1).unwrap();
    let public_key = lines[1].split(": ").nth(1).unwrap();

    if let Some(path) = private_key_path {
        let mut file = create_all_path(path).expect("Failed to create file");
        file.write_all(private_key.as_bytes())
            .expect("Failed to write to file");
    }
    if let Some(path) = public_key_path {
        let mut file = create_all_path(path).expect("Failed to create file");
        file.write_all(public_key.as_bytes())
            .expect("Failed to write to file");
    }
    Ok(())
}

fn main() {
    let private = PathBuf::from("keys/private.key");
    let public = PathBuf::from("keys/public.key");
    let _ = create_keys(Some(&private), Some(&public));
}
