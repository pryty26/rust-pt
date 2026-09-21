// File: rust_pt\pt_core\src\orport\protocol.rs
// Directory: rust_pt\pt_core\src\orport
// Filename: protocol.rs
//======================================================================

use super::extorport::ExtOrPort;
use super::extorport::ExtOrPortReply;
use super::traits::{ClientExtOrPortProtocol, ClientRecvExtOrPortProtocol};
use crate::variables::{CMD_DONE, CMD_TRANSPORT, CMD_USERADDR};
use async_trait::async_trait;
use pt_err::ExtOrPortError;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

impl ExtOrPort {
    /// As of September 2026, the pt-spec does not state when the server
    /// should return `OKAY` or `DENY`.
    /// But in C-tor and goptlib, code shows that server will return `Okay` after sending `done`
    /// Therefore, this function is here
    /// # Errors
    /// - if tokio `write_all(...)` returns Error
    /// - if the stream is missing
    /// - [`ExtOrPortError::StreamMissing`] if the reader half is not set,
    ///   i.e. `connect()` has not completed successfully.
    /// - Any error returned by `PtTracing::info`, typically when
    ///   `PT_TRACING` is unset.
    /// - [`ExtOrPortError::Other`] if reading from the server fails,
    ///   including `UnexpectedEof` when the server closes the connection
    ///   mid-frame.
    /// - [`ExtOrPortError::Other`] if the command or length bytes cannot be
    ///   converted to `u16` / `ExtOrPortReply`.
    pub async fn done_wait(&mut self) -> Result<ExtOrPortReply, ExtOrPortError> {
        let writer = self.writer.as_mut().ok_or(ExtOrPortError::StreamMissing)?;
        Self::done(writer).await?;
        self.recv_listen().await
    }
}
// For docs to these functions
// See traits
#[async_trait]
impl ClientExtOrPortProtocol for ExtOrPort {
    async fn done(writer: &mut OwnedWriteHalf) -> Result<(), ExtOrPortError> {
        let msg = CMD_DONE.to_be_bytes();
        let body_len = 0x0000_u16.to_be_bytes();
        writer.write_all(&msg).await?;
        writer.write_all(&body_len).await?;
        Ok(())
    }
    async fn user_addr(
        writer: &mut OwnedWriteHalf,
        client_addr: String,
    ) -> Result<(), ExtOrPortError> {
        // An ASCII string holding the TCP/IP address of the client of the
        // pluggable transport proxy. A Tor bridge SHOULD use that address to
        // collect statistics about its clients.  Recognized formats are:
        //   1.2.3.4:5678
        //   [1:2::3:4]:5678
        // (Current Tor versions may accept other formats, but this is a bug: transports MUST NOT send them.)
        SocketAddr::from_str(&client_addr)
            .map_err(|e| ExtOrPortError::InvalidUserAddr(e.to_string()))?;
        let msg = CMD_USERADDR.to_be_bytes();
        let body_len = u16::try_from(client_addr.len())
            .map_err(|e| anyhow::anyhow!(e))?
            .to_be_bytes();
        writer.write_all(&msg).await?;
        writer.write_all(&body_len).await?;
        writer.write_all(client_addr.as_bytes()).await?;
        Ok(())
    }
    async fn transport(writer: &mut OwnedWriteHalf, pt_name: String) -> Result<(), ExtOrPortError> {
        let msg = CMD_TRANSPORT.to_be_bytes();
        let body_len: [u8; 2] = u16::try_from(pt_name.len())
            .map_err(|e| ExtOrPortError::PtNameTooLong(e.to_string()))?
            .to_be_bytes();
        writer.write_all(&msg).await?;
        writer.write_all(&body_len).await?;
        writer.write_all(pt_name.as_bytes()).await?;
        Ok(())
    }
}
