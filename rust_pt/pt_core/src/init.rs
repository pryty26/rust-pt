// File: rust_pt\pt_core\src\init.rs
// Directory: rust_pt\pt_core\src
// Filename: init.rs
//======================================================================

/*
use pt_config::configs::core::ConfigKey;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use pt_tracing::{prelude::*};
use anyhow::Result;

/// Init the Pt
/// #NOTE
/// Before calling this function, please call:
/// `PtTracing::fmt().with_severity(severity).try_init()?;`
/// # Errors
/// - if user haven't inited `PtTracing`
/// # Panics
/// -  if user haven't inited `PtTracing`
/// -  if environment are not set or contains incorrect settings
pub fn try_init() -> Result<()> {
    PtTracing::notice("Launching the Pt")?; // Could Panic (See docs of notice())
    let config_key = match ConfigKey::init(){

    } // Could Panic (See docs of init())

    Ok(())
}
    */
