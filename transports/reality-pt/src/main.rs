// @@ begin lint list
#![allow(renamed_and_removed_lints)] // @@REMOVE_WHEN(ci_arti_stable)
#![allow(unknown_lints)] // @@REMOVE_WHEN(ci_arti_nightly)
#![allow(clippy::cognitive_complexity)] // See arti#2556
#![allow(clippy::collapsible_if)] // See arti#2342
#![allow(clippy::let_unit_value)] // This can reasonably be done for explicitness
#![allow(clippy::needless_lifetimes)] // See arti#1765
#![allow(clippy::needless_raw_string_hashes)] // complained-about code is fine, often best
#![allow(clippy::result_large_err)] // temporary workaround for arti#587
#![allow(clippy::significant_drop_in_scrutinee)] // arti/-/merge_requests/588/#note_2812945
#![allow(clippy::uninlined_format_args)]
#![allow(mismatched_lifetime_syntaxes)] // temporary workaround for arti#2060
#![warn(missing_docs)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(clippy::all)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_option)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unseparated_literal_suffix)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::checked_conversions)]
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::exhaustive_enums)]
#![deny(clippy::exhaustive_structs)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::fallible_impl_from)]
#![deny(clippy::implicit_clone)]
#![deny(clippy::large_stack_arrays)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::print_stderr)]
#![deny(clippy::print_stdout)]
#![deny(clippy::ref_option_ref)]
#![deny(clippy::string_slice)] // See arti#2571
#![deny(clippy::unchecked_time_subtraction)]
#![deny(clippy::unnecessary_wraps)]
#![deny(clippy::unused_async)]
#![deny(clippy::unwrap_used)]
//! <!-- @@ end lint list
use anyhow::Result;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
pub mod variable;
use variable::XRAY_CMD;
/// Contains function for downloading and even starting the Xray
pub mod download;
pub use download::get_xray;
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
        },
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
