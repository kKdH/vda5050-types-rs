//!
//! This crate provides data-types defined by the [VDA5050](https://github.com/VDA5050/VDA5050)
//! standard. VDA5050 is an open standard for communication between AGV fleets and a central master control.
//!
//! # Crate Features
//!
//! Enable or disable features according to your needs and in order to optimize for compile time and space.
//!
//! | Feature   | Default  | Description                                                                                                            |
//! | --------- |:--------:| ---------------------------------------------------------------------------------------------------------------------- |
//! | fmt       | &#x2714; | When enabled, certain types will provide an implementation for [`core::fmt::Debug`] and [`core::fmt::Display`] traits. |
//! | serde     | &#x2717; | When enabled, certain types will provide an implementation for [`serde::Serialize`] and [`serde::Deserialize`] traits. |
//! | v2_0      | &#x2717; | When enabled, VDA5050 version 2 types are available.                                                                   |
//!
//! <sup>&#x2714; enabled, &#x2717; disabled</sup>
//!
#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
extern crate std;

extern crate alloc;

mod action;
mod common;
mod connection;
mod factsheet;
mod instant_actions;
mod order;
mod state;
mod visualization;

#[cfg(any(feature = "v2_0", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_0")))]
pub mod v2_0 {

    pub mod common {
        pub use crate::action::Action as Action;
        pub use crate::action::ActionParameter as ActionParameter;
        pub use crate::action::BlockingType as BlockingType;

        pub use crate::common::AgvPosition as AgvPosition;
        pub use crate::common::BoundingBoxReference as BoundingBoxReference;
        pub use crate::common::ControlPoint as ControlPoint;
        pub use crate::common::HeaderId as HeaderId;
        pub use crate::common::LoadDimensions as LoadDimensions;
        pub use crate::common::NodePosition as NodePosition;
        pub use crate::common::Timestamp as Timestamp;
        pub use crate::common::Trajectory as Trajectory;
        pub use crate::common::Velocity as Velocity;
    }

    pub mod connection {
        pub use crate::connection::Connection as Connection;
        pub use crate::connection::ConnectionState as ConnectionState;
    }

    pub mod factsheet {
        pub use crate::factsheet::ActionParameter as ActionParameter;
        pub use crate::factsheet::ActionScope as ActionScope;
        pub use crate::factsheet::AgvAction as AgvAction;
        pub use crate::factsheet::AgvClass as AgvClass;
        pub use crate::factsheet::AgvGeometry as AgvGeometry;
        pub use crate::factsheet::AgvKinematic as AgvKinematic;
        pub use crate::factsheet::Data as Data;
        pub use crate::factsheet::Envelopes2d as Envelopes2d;
        pub use crate::factsheet::Envelopes3d as Envelopes3d;
        pub use crate::factsheet::Factsheet as Factsheet;
        pub use crate::factsheet::LoadSet as LoadSet;
        pub use crate::factsheet::LoadSpecification as LoadSpecification;
        pub use crate::factsheet::LocalizationType as LocalizationType;
        pub use crate::factsheet::MaxArrayLens as MaxArrayLens;
        pub use crate::factsheet::MaxStringLens as MaxStringLens;
        pub use crate::factsheet::NavigationType as NavigationType;
        pub use crate::factsheet::OptionalParameter as OptionalParameter;
        pub use crate::factsheet::PhysicalParameters as PhysicalParameters;
        pub use crate::factsheet::PolygonPoint as PolygonPoint;
        pub use crate::factsheet::Position as Position;
        pub use crate::factsheet::ProtocolFeatures as ProtocolFeatures;
        pub use crate::factsheet::ProtocolLimits as ProtocolLimits;
        pub use crate::factsheet::Support as Support;
        pub use crate::factsheet::Timing as Timing;
        pub use crate::factsheet::TypeSpecification as TypeSpecification;
        pub use crate::factsheet::ValueDataType as ValueDataType;
        pub use crate::factsheet::WheelDefinition as WheelDefinition;
        pub use crate::factsheet::WheelType as WheelType;
    }

    pub mod instant_actions {
        pub use crate::instant_actions::InstantActions as InstantActions;
    }

    pub mod order {
        pub use crate::order::Edge as Edge;
        pub use crate::order::Node as Node;
        pub use crate::order::Order as Order;
        pub use crate::order::OrientationType as OrientationType;
    }

    pub mod state {
        pub use crate::state::ActionState as ActionState;
        pub use crate::state::BatteryState as BatteryState;
        pub use crate::state::EdgeState as EdgeState;
        pub use crate::state::Error as Error;
        pub use crate::state::ErrorReference as ErrorReference;
        pub use crate::state::ErrorLevel as ErrorLevel;
        pub use crate::state::EStop as EStop;
        pub use crate::state::Information as Information;
        pub use crate::state::InfoReference as InfoReference;
        pub use crate::state::InfoLevel as InfoLevel;
        pub use crate::state::Load as Load;
        pub use crate::state::NodeState as NodeState;
        pub use crate::state::OperatingMode as OperatingMode;
        pub use crate::state::SafetyState as SafetyState;
        pub use crate::state::State as State;
    }

    pub mod visualization {
        pub use crate::visualization::Visualization;
    }
}
