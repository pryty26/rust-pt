// File: transports\aspt\src\main.rs
// Directory: transports\aspt\src
// Filename: main.rs
//======================================================================

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
#![deny(clippy::pedantic)] // This is not in Arti
//! <!-- @@ end lint list

#![allow(clippy::pedantic)]
#![allow(clippy::print_stdout)]
#![allow(unused_variables)]
#![allow(clippy::missing_docs_in_private_items)]
use std::net::SocketAddr;
use tokio::net::TcpListener;
// this is an example, so code quality doesn't matter
use pt_config::configs::{
    core::ConfigKey,
    keys::{ClientKey, CommonKey, ServerKey},
};
use pt_core::{OrPortKind, prelude::*};
use pt_err::PtError;
// Note: Do not use `pt_tracing::PtTracing`
// use prelude instead
use pt_tracing::{SEVERITY, prelude::*};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut pt: Pt = Pt::builder().with_severity(SEVERITY::INFO);
    pt.try_init()?;
    if pt.is_server()? {
        let server_config: &ServerKey = pt.get_server_config()?;
        let server_transports = server_config.TOR_PT_SERVER_TRANSPORTS.clone();
        for transport in server_transports.iter() {
            if transport != "super_cool_launch" {
                PtTracing::smethod_error(transport, "Unsupported transport");
            }
            let addr = "127.0.0.1:2837".parse::<SocketAddr>()?;
            super_cool_launch(addr).await?;
            // ( PTs should launch their PT here, and check whether they supports that transport)
            PtTracing::smethod(transport, &addr.to_string(), None);
            match pt.connect_or().await? {
                OrPortKind::ExtOrPort => {
                    // Finish Sends the full `ExtOrPort` handshake for a new client connection:
                    // `TRANSPORT` → `USERADDR` → `DONE`/`OKAY`.
                    pt.finish(transport.into(), addr.to_string()).await?;
                },
                OrPortKind::OrPort => {},
            }
        }
    }
    // I know I can just use `else`
    // But this is for showing that `is_client()` is also supported
    if pt.is_client()? {
        PtTracing::info("In the client")?;
        // Note that we will automatically send `Proxy Done` and `Version`
        // ( I have wrote a lot of docs so you can read them )
        let client_config: &ClientKey = pt.get_client_config()?;
        let common_config: &CommonKey = pt.get_common_config()?;
        // they may return Errors that are defined in the pt_err
        // # Errors
        // Returns an error
        // - `Pt` is not initialized
        // - the config is not for a client.
        // These are in the docs comment too.
        let client_result: Result<&ClientKey, PtError> = pt.get_client_config();
        match client_result {
            Ok(_) => {},
            Err(PtError::NotInClient(string)) => {
                // Here is the log system,
                // inherently compatible with pt-spec format
                PtTracing::error(&format!("Not in the client {string}"))?;
            },
            Err(_) => {
                // Other errors
            },
        }
        // Or, get a whole config
        let whole_config: &ConfigKey = pt.get()?;

        // The name of these structures' variables follows directly pt-spec
        let client_transports: &Vec<String> = &client_config.TOR_PT_CLIENT_TRANSPORTS;

        // Or you can `clone()` them
        let client_config_clone: ClientKey = pt.get_client_config()?.clone();
    }
    Ok(())
}

async fn super_cool_launch(addr: SocketAddr) -> anyhow::Result<SocketAddr> {
    TcpListener::bind(addr).await?;
    Ok(addr)
}
