use async_trait::async_trait;
use pt_err::ExtOrPortError;
use tokio::net::TcpStream;
/// Protocol
///
/// The extended server port protocol is as follows:
///
///      COMMAND [2 bytes, big-endian]
///      BODYLEN [2 bytes, big-endian]
///      BODY [BODYLEN bytes]
///
///      Commands sent from the transport proxy to the bridge are:
///
///      [0x0000] DONE: There is no more information to give. The next
///        bytes sent by the transport will be those tunneled over it.
///        (body ignored)
///
///      [0x0001] USERADDR: an address:port string that represents the
///        client's address.
///
///      [0x0002] TRANSPORT: a string of the name of the pluggable
///        transport currently in effect on the connection.
///
///      Replies sent from tor to the proxy are:
///
///      [0x1000] OKAY: Send the user's traffic. (body ignored)
///
///      [0x1001] DENY: Tor would prefer not to get more traffic from
///        this address for a while. (body ignored)
///
///      [0x1002] CONTROL: (Not used)
///
///   Parties MUST ignore command codes that they do not understand.

// These protocol's sending shouldn't contain any error,
// thus Result<_, _> is not needed
#[async_trait]
pub trait ClientExtOrPortProtocol {
    /// [0x0000] DONE: There is no more information to give. The next
    /// bytes sent by the transport will be those tunneled over it.
    /// (body ignored)
    async fn done(stream: &mut TcpStream) -> Result<(), ExtOrPortError>;
    ///      [0x0001] USERADDR: an address:port string that represents the
    ///        client's address.
    async fn user_addr(stream: &mut TcpStream, client_addr: String) -> Result<(), ExtOrPortError>;
    ///      [0x0002] TRANSPORT: a string of the name of the pluggable
    ///        transport currently in effect on the connection.
    async fn transoprt(stream: &mut TcpStream, pt_name: String) -> Result<(), ExtOrPortError>;
}
