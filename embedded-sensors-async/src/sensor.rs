//! Async Sensor API
//!
//! This module contains traits generic to all sensors.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

pub use embedded_sensors_hal::sensor::{Error, ErrorKind, ErrorType};

// Re-export the unified threshold traits macro from the blocking crate.
// The async crate uses the `async` mode to generate async versions of the traits.
pub use embedded_sensors_hal::decl_threshold_traits;
