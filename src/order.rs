use alloc::string::String;
use alloc::vec::Vec;

use crate::action::Action;
use crate::common::{HeaderId, NodePosition, Timestamp, Trajectory};

/// An order to be communicated from master control to the AGV.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Order {
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
    /// Unique order Identification.
    pub order_id: String,
    /// orderUpdate identification. Is unique per order_id. If an order update is rejected, this field is to be passed in the rejection message.
    pub order_update_id: u64,
    /// Unique identifier of the zone set that the AGV has to use for navigation or that was used by MC for planning. Optional: Some MC systems do not use zones. Some AGVs do not understand zones. Do not add to message if no zones are used.
    pub zone_set_id: Option<String>,
    /// This list holds the base and the horizon nodes of the order graph.
    pub nodes: Vec<Node>,
    /// Base and Horizon Edges of the Order Graph.
    pub edges: Vec<Edge>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Node {
    /// Unique node identification. For example: pumpenhaus_1, MONTAGE
    pub node_id: String,
    /// Id to track the sequence of nodes and edges in an order and to simplify order updates. The main purpose is to distinguish between a node which is passed more than once within one order_id. The variable sequence_id can run across all nodes and edges of the same order and is reset when a new order_id is issued.
    pub sequence_id: u64,
    /// Verbose Node Description.
    pub node_description: Option<String>,
    /// If true, the node is part of the base plan. If false, the node is part of the horizon plan.
    pub released: bool,
    /// Defines the position on a map in world coordinates. Each floor has its own map. Precision is up to the specific implementation.
    pub node_position: Option<NodePosition>,
    /// Array of actions that are to be executed on the node. Their sequence in the list governs their sequence of execution.
    pub actions: Vec<Action>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Edge {
    /// Unique edge identification
    pub edge_id: String,
    /// Id to track the sequence of nodes and edges in an order and to simplify order updates. The variable sequence_id runs across all nodes and edges of the same order and is reset when a new order_id is issued.
    pub sequence_id: u64,
    /// Verbose description of the edge.
    pub edge_description: Option<String>,
    /// If true, the edge is part of the base plan. If false, the edge is part of the horizon plan.
    pub released: bool,
    /// The node_id of the start node.
    pub start_node_id: String,
    /// The node_id of the end node.
    pub end_node_id: String,
    /// permitted maximum speed of the agv on the edge in m/s. Speed is defined by the fastest point of the vehicle.
    pub max_speed: Option<f32>,
    /// Permitted maximum height of the vehicle, including the load, on edge. In meters.
    pub max_height: Option<f32>,
    /// Permitted minimal height of the edge measured at the bottom of the load. In meters.
    pub min_height: Option<f32>,
    /// Orientation of the AGV on the edge relative to the map coordinate origin (for holonomic vehicles with more than one driving direction). Example: orientation Pi/2 rad will lead to a rotation of 90 degrees. If AGV starts in different orientation, rotate the vehicle on the edge to the desired orientation if rotation_allowed is set to "true". If rotation_allowed is "false", rotate before entering the edge. If that is not possible, reject the order. If a trajectory with orientation is defined, follow the trajectories orientation. If a trajectory without orientation and the orientation field here is defined, apply the orientation to the tangent of the trajectory.
    pub orientation: Option<f32>,
    /// Orientation type of the edge.
    pub orientation_type: Option<OrientationType>,
    /// Sets direction at junctions for line-guided vehicles, to be defined initially (vehicle-individual). Can be descriptive (left, right, middle, straight) or a frequency ("433MHz").
    pub direction: Option<String>,
    /// If true, rotation is allowed on the edge.
    pub rotation_allowed: Option<bool>,
    /// Maximum rotation speed in rad/s
    pub max_rotation_speed: Option<f32>,
    /// Distance of the path from startNode to endNode in meters. Optional: This value is used by line-guided AGVs to decrease their speed before reaching a stop position.
    pub length: Option<f32>,
    /// Trajectory JSON-object for this edge as a NURBS. Defines the curve on which the AGV should move between startNode and endNode. Optional: Can be omitted if AGV cannot process trajectories or if AGV plans its own trajectory.
    pub trajectory: Option<Trajectory>,
    /// Array of action objects with detailed information.
    pub actions: Vec<Action>
}

#[derive(Default)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum OrientationType {
    /// Relative to the global project specific map coordinate system.
    Global,
    /// Tangential to the edge.
    #[default]
    Tangential
}
