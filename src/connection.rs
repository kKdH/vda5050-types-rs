use alloc::string::String;
use crate::common::{HeaderId, Timestamp};

/// AGV connection state reported as a last will message. Has to be sent with retain flag. Once the AGV comes online, it has to send this message on its connect topic, with the connection_state enum set to "ONLINE". The last will message is to be configured with the connection state set to "CONNECTIONBROKEN". Thus, if the AGV disconnects from the broker, master control gets notified via the topic "connection". If the AGV is disconnecting in an orderly fashion (e.g. shutting down, sleeping), the AGV is to publish a message on this topic with the connection_state set to "OFFLINE".
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Connection {
    /// header_id of the message. The header_id is defined per topic and incremented by 1 with each sent (but not necessarily received) message.
    pub header_id: HeaderId,
    /// Timestamp (ISO8601, UTC); YYYY-MM-DDTHH:mm:ss.ssZ; e.g. 2017-04-15T11:40:03.12Z
    pub timestamp: Timestamp,
    /// Version of the protocol [Major].[Minor].[Patch], e.g. 1.3.2
    pub version: String,
    /// Manufacturer of the AGV
    pub manufacturer: String,
    /// Serial number of the AGV
    pub serial_number: String,
    /// Connection state.
    pub connection_state: ConnectionState
}

/// Connection state.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ConnectionState {
    /// The Connection between AGV and broker is active.
    Online,
    /// The Connection between AGV and broker has gone offline in a coordinated way.
    Offline,
    /// The connection between AGV and broker has unexpectedly ended.
    ConnectionBroken
}
