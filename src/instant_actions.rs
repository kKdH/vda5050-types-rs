use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use crate::action::Action;
use crate::common::{HeaderId, Timestamp};

/// Instant actions that the AGV is to execute as soon as they arrive.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct InstantActions {
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
    /// Array of actions that need to be performed immediately and are not part of the regular order.
    pub instant_actions: Vec<Action>
}
