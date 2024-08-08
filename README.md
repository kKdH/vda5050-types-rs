# VDA5050 Types
This crate provides data-types for the rust programming language defined by the [VDA5050](https://github.com/VDA5050/VDA5050)
standard. VDA5050 is an open standard for communication between AGV fleets and a central master control.

## Usage

Add a dependency on this crate to your project's Cargo.toml:
```toml
[dependencies]
vda5050-types = { version = "0.1.0", features = ["v2_0"] }
```

```rust
use vda5050_types::v2_0::common::{Action, BlockingType};
use vda5050_types::v2_0::instant_actions::InstantActions;

fn main() {
    let action = InstantActions {
        header_id: 0,
        timestamp: Utc::now(),
        version: String::from("2"),
        manufacturer: String::from("Fubar Co."),
        serial_number: String::from("1234"),
        instant_actions: vec![
            Action {
                action_type: String::from("pick"),
                action_id: String::from("pick-1"),
                action_description: None,
                blocking_type: BlockingType::Soft,
                action_parameters: vec![],
            }
        ],
    };
}
```

## [Documentation](https://docs.rs/vda5050-types)

## License
Licensed using the [MIT](LICENSE).

## Contributing

All contributions are welcome. Any contribution intentionally submitted for inclusion in this crate by you, as defined in the [MIT license](LICENSE), shall be licensed without any additional terms or conditions.
