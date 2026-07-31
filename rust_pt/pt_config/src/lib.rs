#[doc(hidden)]
pub use derive_deftly;
pub mod configs;
pub mod macros;
use once_cell::sync::Lazy;
use std::env;
/// Init the .env
pub static ENV_LOADED: Lazy<()> = Lazy::new(|| {
    let _ = dotenv();
    tracing::debug!(".env loaded");
});
/// SideMode which declaires which side the service is running
pub enum SideMode {
    Client,
    Server,
    Unknown
}
impl Into<String> for SideMode {
    /// implement into String for SideMode
    /// If the SideMode is Unknown, we will inquire user to set it
    /// TODO(pryty26):
    /// We should implement that via derive-deftly for reducing code
    fn into(self) -> String {
        match self {
            SideMode::Client => "Client".to_string(),
            SideMode::Server => "Server".to_string(),
            SideMode::Unknown => "Unknown: Missing or invalid SIDEMODE in .env. Please set to 'client' or 'server'".to_string()            
        }
    }
}

/// Get SiteMode from the .env
/// TODO(pryty26):
/// We should add log!(When the pt_tracing is done)
const MODE: Lazy<SideMode> = Lazy::new(||{
    let _ = &*ENV_LOADED;
    env::var("SIDEMODE").unwrap_or(||"")
})