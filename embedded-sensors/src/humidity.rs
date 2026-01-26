//! Blocking Humidity Sensor API
//!
//! This API provides generic methods for interfacing with humidity sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the RelativeHumiditySensor trait for a humidity sensor.
//!
//! ```
//! use embedded_sensors_hal::sensor;
//! use embedded_sensors_hal::humidity::{RelativeHumiditySensor, Percentage};
//!
//! // A struct representing a humidity sensor.
//! pub struct MyHumiditySensor {
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
//! impl sensor::ErrorType for MyHumiditySensor {
//!     type Error = Error;
//! }
//!
//! impl RelativeHumiditySensor for MyHumiditySensor {
//!     fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//! ```

use crate::sensor::ErrorType;

/// Associates the units relative humidity (RH) samples are measured in with the underlying data type.
pub type Percentage = f32;

/// Blocking Relative Humidity Sensor methods.
pub trait RelativeHumiditySensor: ErrorType {
    /// Returns a relative humidity (RH) sample as a percentage.
    fn relative_humidity(&mut self) -> Result<Percentage, Self::Error>;
}

impl<T: RelativeHumiditySensor + ?Sized> RelativeHumiditySensor for &mut T {
    #[inline]
    fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
        T::relative_humidity(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::{Error, ErrorKind};

    // Mock test values
    const TEST_HUMIDITY_1: Percentage = 65.0;
    const TEST_HUMIDITY_2: Percentage = 50.5;

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct MockHumiditySensor {
        value: Percentage,
    }

    impl crate::sensor::ErrorType for MockHumiditySensor {
        type Error = MockError;
    }

    impl RelativeHumiditySensor for MockHumiditySensor {
        fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
            Ok(self.value)
        }
    }

    #[test]
    fn test_humidity_sensor_trait() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY_1,
        };
        let result = sensor.relative_humidity();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_HUMIDITY_1);
    }

    #[test]
    fn test_humidity_sensor_trait_mut_ref() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY_2,
        };
        let mut_ref = &mut sensor;
        let result = mut_ref.relative_humidity();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_HUMIDITY_2);
    }
}
