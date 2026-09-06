// File: rust_pt\pt_core\src\orport\protocol.rs
// Directory: rust_pt\pt_core\src\orport
// Filename: protocol.rs
//======================================================================

use super::extorport::ExtOrPort;
use super::traits::ClientExtOrPortProtocol;
use async_trait::async_trait;
use pt_err::ExtOrPortError;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

// For docs to these functions
// See traits
#[async_trait]
impl ClientExtOrPortProtocol for ExtOrPort {
    async fn done(stream: &mut TcpStream) -> Result<(), ExtOrPortError> {
        let msg = 0x0000_u16.to_be_bytes();
        let body_len = 0x0000_u16.to_be_bytes();
        stream.write_all(&msg).await?;
        stream.write_all(&body_len).await?;
        Ok(())
    }
    async fn user_addr(stream: &mut TcpStream, client_addr: String) -> Result<(), ExtOrPortError> {
        // An ASCII string holding the TCP/IP address of the client of the
        // pluggable transport proxy. A Tor bridge SHOULD use that address to
        // collect statistics about its clients.  Recognized formats are:
        //   1.2.3.4:5678
        //   [1:2::3:4]:5678
        // (Current Tor versions may accept other formats, but this is a bug: transports MUST NOT send them.)
        SocketAddr::from_str(&client_addr)
            .map_err(|e| ExtOrPortError::InvalidUserAddr(e.to_string()))?;
        let msg = 0x0001_u16.to_be_bytes();
        let body_len = client_addr.len().to_be_bytes();
        stream.write_all(&msg).await?;
        stream.write_all(&body_len).await?;
        stream.write_all(client_addr.as_bytes()).await?;
        Ok(())
    }
    async fn transport(stream: &mut TcpStream, pt_name: String) -> Result<(), ExtOrPortError> {
        let msg = 0x0002_u16.to_be_bytes();
        let body_len = pt_name.len().to_be_bytes();
        stream.write_all(&msg).await?;
        stream.write_all(&body_len).await?;
        stream.write_all(pt_name.as_bytes()).await?;
        Ok(())
    }
}
