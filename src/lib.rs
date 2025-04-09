#![deny(missing_docs)]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "README.md" ) ) ]

mod colors;
mod components;

/// Horizontal or vertical bar
pub use components::Bar;
/// Gauge
pub use components::Gauge;
/// Toggle switch style
pub use components::ToggleStyle;
/// Toggle switch
pub use components::ToggleSwitch;
