use alloc::string::String;
use alloc::vec::Vec;
use crate::common::{BoundingBoxReference, HeaderId, LoadDimensions, Timestamp};

/// The factsheet provides basic information about a specific AGV type series. This information allows comparison of different AGV types and can be applied for the planning, dimensioning and simulation of an AGV system. The factsheet also includes information about AGV communication interfaces which are required for the integration of an AGV type series into a VD[M]A-5050-compliant master control.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Factsheet {
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
    /// These parameters generally specify the class and the capabilities of the AGV
    pub type_specification: Option<TypeSpecification>,
    /// These parameters specify the basic physical properties of the AGV
    pub physical_parameters: Option<PhysicalParameters>,
    /// This JSON-object describes the protocol limitations of the AGV. If a parameter is not defined or set to zero then there is no explicit limit for this parameter.
    pub protocol_limits: Option<ProtocolLimits>,
    /// Supported features of VDA5050 protocol
    pub protocol_features: Option<ProtocolFeatures>,
    /// Detailed definition of AGV geometry
    pub agv_geometry: Option<AgvGeometry>,
    /// Abstract specification of load capabilities
    pub load_specification: Option<LoadSpecification>,
    /// Detailed specification of localization
    pub localization_parameters: Option<u64>
}

/// These parameters generally specify the class and the capabilities of the AGV.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct TypeSpecification {
    /// Free text generalized series name as specified by manufacturer
    pub series_name: String,
    /// Free text human-readable description of the AGV type series
    pub series_description: Option<String>,
    /// simplified description of AGV kinematics-type.
    pub agv_kinematic: AgvKinematic,
    /// Simplified description of AGV class.
    pub agv_class: AgvClass,
    /// maximum loadable mass
    pub max_load_mass: f32,
    /// simplified description of localization type
    pub localization_types: Vec<LocalizationType>,
    /// List of path planning types supported by the AGV, sorted by priority
    pub navigation_types: Vec<NavigationType>
}

/// Simplified description of AGV kinematics-type.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum AgvKinematic {
    Diff,
    Omni,
    #[cfg_attr(feature = "serde", serde(rename = "THREEWHEEL"))]
    ThreeWheel
}

/// Simplified description of AGV class.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum AgvClass {
    Forklift,
    Conveyor,
    Tugger,
    Carrier
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum LocalizationType {
    Natural,
    Reflector,
    Rfid,
    Dmc,
    Spot,
    Grid
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum NavigationType {
    PhysicalLindeGuided,
    VirtualLineGuided,
    Autonomous
}

/// These parameters specify the basic physical properties of the AGV.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PhysicalParameters {
    /// minimal controlled continuous speed of the AGV
    pub speed_min: f32,
    /// maximum speed of the AGV
    pub speed_max: f32,
    /// maximum acceleration with maximum load
    pub acceleration_max: f32,
    /// maximum deceleration with maximum load
    pub deceleration_max: f32,
    /// minimum height of AGV
    pub height_min: Option<f32>,
    /// maximum height of AGV
    pub height_max: f32,
    /// width of AGV
    pub width: f32,
    /// length of AGV
    pub length: f32
}

/// This JSON-object describes the protocol limitations of the AGV. If a parameter is not defined or set to zero then there is no explicit limit for this parameter.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ProtocolLimits {
    /// maximum lengths of strings
    pub max_string_lens: MaxStringLens,
    /// maximum lengths of arrays
    pub max_array_lens: MaxArrayLens,
    /// timing information
    pub timing: Timing
}

/// Maximum lengths of strings
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct MaxStringLens {
    /// maximum MQTT Message length
    pub msg_len: Option<u64>,
    /// maximum length of serial-number part in MQTT-topics. Affected Parameters: order.serial_number, instantActions.serial_number, state.SerialNumber, visualization.serial_number, connection.serial_number
    pub topic_serial_len: Option<u64>,
    /// maximum length of all other parts in MQTT-topics. Affected parameters: order.timestamp, order.version, order.manufacturer, instantActions.timestamp, instantActions.version, instantActions.manufacturer, state.timestamp, state.version, state.manufacturer, visualization.timestamp, visualization.version, visualization.manufacturer, connection.timestamp, connection.version, connection.manufacturer
    pub topic_elem_len: Option<u64>,
    /// maximum length of ID-Strings. Affected parameters: order.orderId, order.zoneSetId, node.nodeId, nodePosition.mapId, action.actionId, edge.edgeId, edge.startNodeId, edge.endNodeId
    pub id_len: Option<u64>,
    /// If true ID-strings need to contain numerical values only
    pub id_numerical_only: Option<bool>,
    /// maximum length of ENUM- and Key-Strings. Affected parameters: action.actionType, action.blockingType, edge.direction, actionParameter.key, state.operatingMode, load.loadPosition, load.loadType, actionState.actionStatus, error.errorType, error.errorLevel, errorReference.referenceKey, info.infoType, info.infoLevel, safetyState.eStop, connection.connectionState
    pub enum_len: Option<u64>,
    /// maximum length of loadId Strings
    pub load_id_len: Option<u64>
}

/// Maximum lengths of arrays.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct MaxArrayLens {
    /// Maximum number of nodes per order processable by the AGV
    #[cfg_attr(feature = "serde", serde(rename = "order.nodes"))]
    pub order_nodes: u32,
    /// Maximum number of edges per order processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "order.edges"))]
    pub order_edges: u32,
    /// Maximum number of actions per node processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "node.actions"))]
    pub node_actions: u32,
    /// Maximum number of actions per edge processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "edge.actions"))]
    pub edge_actions: u32,
    /// Maximum number of parameters per action processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "actions.actionsParameters"))]
    pub actions_actions_parameters: u32,
    /// Maximum number of instant actions per message processable by the AGV
    #[cfg_attr(feature = "serde", serde(rename = "instantActions"))]
    pub instant_actions: u32,
    /// Maximum number of knots per trajectory processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "trajectory.knotVector"))]
    pub trajectory_knot_vector: u32,
    /// Maximum number of control points per trajectory processable by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "trajectory.controlPoints"))]
    pub trajectory_control_points: u32,
    /// Maximum number of nodeStates sent by the AGV, maximum number of nodes in base of AGV.
    #[cfg_attr(feature = "serde", serde(rename = "state.nodeStates"))]
    pub state_node_states: u32,
    /// Maximum number of edgeStates sent by the AGV, maximum number of edges in base of AGV.
    #[cfg_attr(feature = "serde", serde(rename = "state.edgeStates"))]
    pub state_edge_states: u32,
    /// Maximum number of load-objects sent by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "state.loads"))]
    pub state_loads: u32,
    /// Maximum number of actionStates sent by the AGV.
    #[cfg_attr(feature = "serde", serde(rename = "state.actionStates"))]
    pub state_action_states: u32,
    /// Maximum number of errors sent by the AGV in one state-message.
    #[cfg_attr(feature = "serde", serde(rename = "state.errors"))]
    pub state_errors: u32,
    /// Maximum number of information objects sent by the AGV in one state-message.
    #[cfg_attr(feature = "serde", serde(rename = "state.information"))]
    pub state_information: u32,
    /// Maximum number of error references sent by the AGV for each error.
    #[cfg_attr(feature = "serde", serde(rename = "error.errorReferences"))]
    pub error_error_references: u32,
    /// Maximum number of info references sent by the AGV for each information.
    #[cfg_attr(feature = "serde", serde(rename = "information.infoReferences"))]
    pub information_info_references: u32,
}

/// Timing information.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Timing {
    /// minimum interval sending order messages to the AGV
    pub min_order_interval: f32,
    /// minimum interval for sending state-messages
    pub min_state_interval: f32,
    /// default interval for sending state-messages if not defined, the default value from the main document is used
    pub default_state_interval: Option<f32>,
    /// default interval for sending messages on visualization topic
    pub visualization_interval: Option<f32>
}

/// Supported features of VDA5050 protocol
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ProtocolFeatures {
    /// list of supported and/or required optional parameters. Optional parameters, that are not listed here, are assumed to be not supported by the AGV.
    pub optional_parameters: Vec<OptionalParameter>,
    /// list of all actions with parameters supported by this AGV. This includes standard actions specified in VDA5050 and manufacturer-specific actions
    pub agv_actions: Vec<AgvAction>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct OptionalParameter {
    /// full name of optional parameter, e.g. “order.nodes.nodePosition.allowedDeviationTheta”
    pub parameter: String,
    /// type of support for the optional parameter, the following values are possible: SUPPORTED: optional parameter is supported like specified. REQUIRED: optional parameter is required for proper AGV-operation.
    pub support: Support,
    /// free text. Description of optional parameter. E.g. Reason, why the optional parameter ‚direction‘ is necessary for this AGV-type and which values it can contain. The parameter ‘nodeMarker’ must contain unsigned interger-numbers only. Nurbs-Support is limited to straight lines and circle segments.
    pub description: Option<String>
}

/// Type of support for the optional parameter.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum Support {
    /// Optional parameter is supported like specified.
    Supported,
    /// Optional parameter is required for proper AGV-operation.
    Required
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct AgvAction {
    /// unique actionType corresponding to action.actionType
    pub action_type: String,
    /// free text: description of the action
    pub action_description: Option<String>,
    /// list of allowed scopes for using this action-type. INSTANT: usable as instantAction, NODE: usable on nodes, EDGE: usable on edges.
    pub action_scopes: Vec<ActionScope>,
    /// list of parameters. if not defined, the action has no parameters
    pub action_parameters: Vec<ActionParameter>,
    /// free text: description of the resultDescription
    pub result_description: Option<String>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ActionScope {
    Instant,
    Node,
    Edge
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ActionParameter {
    /// key-String for Parameter
    pub key: String,
    /// data type of Value, possible data types are: BOOL, NUMBER, INTEGER, FLOAT, STRING, OBJECT, ARRAY
    pub value_data_type: ValueDataType,
    /// free text: description of the parameter
    pub description: Option<String>,
    /// True: optional parameter
    pub is_optional: Option<bool>
}

/// Data type of Value.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ValueDataType {
    Bool,
    Number,
    Integer,
    Float,
    String,
    Object,
    Array
}

/// Detailed definition of AGV geometry.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct AgvGeometry {
    /// list of wheels, containing wheel-arrangement and geometry
    pub wheel_definitions: Vec<WheelDefinition>,
    pub envelopes2d: Vec<Envelopes2d>,
    /// list of AGV-envelope curves in 3D (german: „Hüllkurven“)
    pub envelopes3d: Vec<Envelopes3d>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct WheelDefinition {
    /// wheel type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub  wheel_type: WheelType,
    /// True: wheel is actively driven (de: angetrieben)
    pub is_active_driven: bool,
    /// True: wheel is actively steered (de: aktiv gelenkt)
    pub is_active_steered: bool,
    pub position: Position,
    /// nominal diameter of wheel
    pub diameter: f32,
    /// nominal width of wheel
    pub width: f32,
    /// nominal displacement of the wheel’s center to the rotation point (necessary for caster wheels). If the parameter is not defined, it is assumed to be 0
    pub center_displacement: Option<f32>,
    /// free text: can be used by the manufacturer to define constraints
    pub constraints: Option<String>
}

/// Type of an AGV's wheel.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum WheelType {
    Drive,
    Caster,
    Fixed,
    Mecanum
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Position {
    /// [m] x-position in AGV-coordinate system
    pub x: f32,
    /// y-position in AGV-coordinate system
    pub y: f32,
    /// orientation of wheel in AGV-coordinate system Necessary for fixed wheels
    pub theta: Option<f32>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Envelopes2d {
    /// name of the envelope curve set
    pub set: String,
    /// envelope curve as a x/y-polygon polygon is assumed as closed and must be non-self-intersecting
    pub polygon_points: Vec<PolygonPoint>,
    /// free text: description of envelope curve set
    pub description: Option<String>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct PolygonPoint {
    /// x-position of polygon-point
    pub x: f32,
    /// y-position of polygon-point
    pub y: f32
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Envelopes3d {
    /// name of the envelope curve set
    pub set: String,
    /// format of data e.g. DXF
    pub format: String,
    /// 3D-envelope curve data, format specified in ‚format‘
    pub data: Option<Data>,
    /// protocol and url-definition for downloading the 3D-envelope curve data e.g. ftp://xxx.yyy.com/ac4dgvhoif5tghji
    pub url: Option<String>,
    /// free text: description of envelope curve set
    pub description: Option<f32>
}

/// 3D-envelope curve data, format specified in ‚format‘
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Data;

/// Abstract specification of load capabilities.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct LoadSpecification {
    /// list of load positions / load handling devices. This lists contains the valid values for the oarameter “state.loads[].loadPosition” and for the action parameter “lhd” of the actions pick and drop. If this list doesn’t exist or is empty, the AGV has no load handling device.
    pub load_positions: Vec<String>,
    /// list of load-sets that can be handled by the AGV
    pub load_sets: Vec<LoadSet>
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct LoadSet {
    /// Unique name of the load set, e.g. DEFAULT, SET1, ...
    pub set_name: String,
    /// type of load e.g. EPAL, XLT1200, ….
    pub load_type: String,
    /// list of load positions btw. load handling devices, this load-set is valid for. If this parameter does not exist or is empty, this load-set is valid for all load handling devices on this AGV.
    pub load_positions: Vec<String>,
    /// bounding box reference as defined in parameter loads[] in state-message
    pub bounding_box_reference: Option<BoundingBoxReference>,
    pub load_dimensions: Option<LoadDimensions>,
    /// maximum weight of loadtype
    pub max_weigth: Option<f32>,
    /// minimum allowed height for handling of this load-type and –weight. References to bounding_box_reference
    pub min_loadhandling_height: Option<f32>,
    /// maximum allowed height for handling of this load-type and –weight. references to bounding_box_reference
    pub max_loadhandling_height: Option<f32>,
    /// minimum allowed depth for this load-type and –weight. references to bounding_box_reference
    pub min_loadhandling_depth: Option<f32>,
    /// maximum allowed depth for this load-type and –weight. references to bounding_box_reference
    pub max_loadhandling_depth: Option<f32>,
    /// minimum allowed tilt for this load-type and –weight
    pub min_loadhandling_tilt: Option<f32>,
    /// maximum allowed tilt for this load-type and –weight
    pub max_loadhandling_tilt: Option<f32>,
    /// maximum allowed speed for this load-type and –weight
    pub agv_speed_limit: Option<f32>,
    /// maximum allowed acceleration for this load-type and –weight
    pub agv_acceleration_limit: Option<f32>,
    /// maximum allowed deceleration for this load-type and –weight
    pub agv_deceleration_limit: Option<f32>,
    /// approx. time for picking up the load
    pub pick_time: Option<f32>,
    /// approx. time for dropping the load
    pub drop_time: Option<f32>,
    /// free text description of the load handling set
    pub description: Option<f32>
}
