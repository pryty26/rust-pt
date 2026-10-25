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
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::option_option)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unseparated_literal_suffix)]
#![deny(clippy::all)]
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
#![deny(missing_docs)]
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


//! Welcome to the official example of rust-pt!
//! ヾ(•ω•`)o
// this is an example, so code quality doesn't matter
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::print_stdout)]
use tokio::net::TcpListener;
use tokio::{io::copy_bidirectional, net::TcpStream};
use pt_config::configs::{
    core::ConfigKey,
    keys::{ClientKey, CommonKey, ServerKey},
};
use pt_core::{OrProdKind, prelude::*};
use pt_core::socks::core::SocksConn;
use pt_err::PtError;
use std::collections::HashSet;
// Note: Do not use `pt_tracing::PtTracing`
// use prelude instead
use pt_tracing::{SEVERITY, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // When `with_sup_transports(...)` is set before `try_init()`,
    // `Pt` validates every transport name and reports
    // `cmethod_error` / `smethod_error` for unsupported ones.
    // Same goes with `with_proxy_schemes(...)`
    let mut pt: Pt = Pt::builder()
        .with_severity(SEVERITY::INFO)
        .with_sup_transports(Some(vec!["cool_transport".to_string()]))
        .with_proxy_schemes(Some(HashSet::from([
            "socks5".to_string(),
            "http".to_string(),
        ])));
    pt.try_init()?;

    if pt.is_server()? {
        run_server(&pt).await;
    }

    // I know I can just use `else`
    // But this is for showing that `is_client()` is also supported
    if pt.is_client()? {
        run_client(&pt).await?;
    }

    Ok(())
}

async fn run_server(pt: &Pt) {
    let server_transports = {
        let server_config: &ServerKey = match pt.get_server_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                PtTracing::error(&e.to_string()).ok();
                return;
            },
        };
        server_config.TOR_PT_SERVER_TRANSPORTS.clone()
    };

    for transport_name in &server_transports {
        let addr = {
            let cfg = match pt.get_server_config() {
                Ok(cfg) => cfg,
                Err(e) => {
                    PtTracing::smethod_error(transport_name, &e.to_string());
                    continue;
                },
            };
            *cfg.TOR_PT_SERVER_BINDADDR
                .get(transport_name.as_str())
                .expect("addr unfound")
        };

        let prod_or = match pt.prod_or() {
            Ok(prod_or) => prod_or,
            Err(e) => {
                PtTracing::smethod_error(transport_name, &e.to_string());
                continue;
            },
        };

        tokio::spawn(async move {
            if let Err(e) = serve_one(addr, prod_or).await {
                PtTracing::error(&e.to_string()).ok();
            }
        });
    }

    std::future::pending::<()>().await;
}

async fn serve_one(addr: std::net::SocketAddr, prod_or: OrProdKind) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut orport_connection = prod_or.connect_or_tcpstream().await?;

    let addr = listener.local_addr()?;
    // ( PTs should launch their PT here, and check whether they supports that transport)
    PtTracing::smethod("cool_transport", &addr.to_string(), None);
    let (mut stream, _addr) = listener.accept().await?;
    copy_bidirectional(&mut orport_connection, &mut stream).await?;
    Ok(())
}

async fn run_client(pt: &Pt) -> anyhow::Result<()> {
    PtTracing::info("In the client")?;
    // Note that we will automatically send `Proxy Done` and `Version`
    // ( I have wrote a lot of docs so you can read them )
    let client_config: &ClientKey = pt.get_client_config()?;
    let _common_config: &CommonKey = pt.get_common_config()?;

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
        Err(e) => {
            // Other errors
            PtTracing::error(&e.to_string())?;
        },
    }
    // Or, get a whole config
    let _whole_config: &ConfigKey = pt.get()?;

    // The name of these structures' variables follows directly pt-spec
    // transports
    let _: &Vec<String> = &client_config.TOR_PT_CLIENT_TRANSPORTS;

    // Or you can `clone()` them
    let _client_config_clone: ClientKey = pt.get_client_config()?.clone();
    // We have to launch a socks port, and the Tor will use it
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    // ( PTs should launch their PT here, and check whether they supports that transport)
    PtTracing::cmethod("cool_transport", "socks5", &addr.to_string());
    loop {
        let (client, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_socks_client(client).await {
                PtTracing::error(&e.to_string()).ok();
            }
        });
    }
}

async fn handle_socks_client(mut client: TcpStream) -> anyhow::Result<()> {
    let mut server_socks_mgr: SocksConn = SocksConn::builder();

    server_socks_mgr.handle_connect(&mut client).await?;

    // Yep, if you uses `String::try_from()`
    // You will get the address that is indicated by socks client
    let target: String = String::try_from(&server_socks_mgr)?;
    let mut upstream = TcpStream::connect(target).await?;
    copy_bidirectional(&mut client, &mut upstream).await?;
    Ok(())
}