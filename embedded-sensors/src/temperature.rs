//! Blocking Temperature Sensor API
//!
//! This API provides generic methods for interfacing with temperature sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the TemperatureSensor trait for a temperature sensor.
//!
//! ```
//! use embedded_sensors_hal::sensor;
//! use embedded_sensors_hal::temperature::{TemperatureSensor, DegreesCelsius};
//!
//! // A struct representing a temperature sensor.
//! pub struct MyTempSensor {
//!     // ...
//! }
//!
//! #[derive(Clone, Copy, Debug)]
//! pub enum Error {
//!     // ...
//! }
//!
//! impl sensor::Error for Error {
//!     fn kind(&self) -> sensor::ErrorKind {
//!         match *self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl sensor::ErrorType for MyTempSensor {
//!     type Error = Error;
//! }
//!
//! impl TemperatureSensor for MyTempSensor {
//!     fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//! ```

use crate::sensor::ErrorType;

/// Associates the units temperature samples are measured in with the underlying data type.
pub type DegreesCelsius = f32;

/// Blocking Temperature Sensor methods.
pub trait TemperatureSensor: ErrorType {
    /// Returns a temperature sample in degrees Celsius.
    fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error>;
}

impl<T: TemperatureSensor + ?Sized> TemperatureSensor for &mut T {
    #[inline]
    fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
        T::temperature(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::{Error, ErrorKind};

    // Mock test value
    const TEST_TEMP: DegreesCelsius = 27.0;

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct MockTempSensor {
        value: DegreesCelsius,
    }

    impl crate::sensor::ErrorType for MockTempSensor {
        type Error = MockError;
    }

    impl TemperatureSensor for MockTempSensor {
        fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
            Ok(self.value)
        }
    }

    #[test]
    fn test_temperature_sensor_trait() {
        let mut sensor = MockTempSensor { value: TEST_TEMP };
        let result = sensor.temperature();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_TEMP);
    }

    #[test]
    fn test_temperature_sensor_trait_mut_ref() {
        let mut sensor = MockTempSensor { value: TEST_TEMP };
        let mut_ref = &mut sensor;
        let result = mut_ref.temperature();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, TEST_TEMP);
    }
}
