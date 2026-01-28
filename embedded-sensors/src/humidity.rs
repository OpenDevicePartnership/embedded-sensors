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

// This macro generates the following blocking threshold traits:
//
// pub trait RelativeHumidityThresholdSet: RelativeHumiditySensor {
//     fn set_relative_humidity_threshold_low(&mut self, threshold: Percentage) -> Result<(), Self::Error>;
//     fn set_relative_humidity_threshold_high(&mut self, threshold: Percentage) -> Result<(), Self::Error>;
// }
//
// pub trait RelativeHumidityHysteresis: RelativeHumidityThresholdSet {
//     fn set_relative_humidity_threshold_hysteresis(&mut self, hysteresis: Percentage) -> Result<(), Self::Error>;
// }
decl_threshold_traits!(
    blocking,
    RelativeHumidity,
    RelativeHumiditySensor,
    Percentage,
    "percentage"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::{Error, ErrorKind};
    use assert_approx_eq::assert_approx_eq;

    // Mock test value
    const TEST_HUMIDITY: Percentage = 65.0;

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct MockHumiditySensor {
        value: Percentage,
        threshold_low: Option<Percentage>,
        threshold_high: Option<Percentage>,
        hysteresis: Option<Percentage>,
    }

    impl crate::sensor::ErrorType for MockHumiditySensor {
        type Error = MockError;
    }

    impl RelativeHumiditySensor for MockHumiditySensor {
        fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
            Ok(self.value)
        }
    }

    impl RelativeHumidityThresholdSet for MockHumiditySensor {
        fn set_relative_humidity_threshold_low(
            &mut self,
            threshold: Percentage,
        ) -> Result<(), Self::Error> {
            self.threshold_low = Some(threshold);
            Ok(())
        }

        fn set_relative_humidity_threshold_high(
            &mut self,
            threshold: Percentage,
        ) -> Result<(), Self::Error> {
            self.threshold_high = Some(threshold);
            Ok(())
        }
    }

    impl RelativeHumidityHysteresis for MockHumiditySensor {
        fn set_relative_humidity_threshold_hysteresis(
            &mut self,
            hysteresis: Percentage,
        ) -> Result<(), Self::Error> {
            self.hysteresis = Some(hysteresis);
            Ok(())
        }
    }

    #[test]
    fn test_humidity_sensor_trait() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let result = sensor.relative_humidity();
        assert!(result.is_ok());
        assert_approx_eq!(result.unwrap(), TEST_HUMIDITY);
    }

    #[test]
    fn test_humidity_sensor_trait_mut_ref() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let result = mut_ref.relative_humidity();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_approx_eq!(value, TEST_HUMIDITY);
    }

    #[test]
    fn test_humidity_threshold_set_low() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let threshold = 50.0;
        let result = sensor.set_relative_humidity_threshold_low(threshold);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.threshold_low.unwrap(), threshold);
    }

    #[test]
    fn test_humidity_threshold_set_high() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let threshold = 80.0;
        let result = sensor.set_relative_humidity_threshold_high(threshold);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.threshold_high.unwrap(), threshold);
    }

    #[test]
    fn test_humidity_threshold_set_mut_ref() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let low_threshold = 40.0;
        let high_threshold = 90.0;

        let result_low = mut_ref.set_relative_humidity_threshold_low(low_threshold);
        assert!(result_low.is_ok());

        let result_high = mut_ref.set_relative_humidity_threshold_high(high_threshold);
        assert!(result_high.is_ok());

        assert_approx_eq!(sensor.threshold_low.unwrap(), low_threshold);
        assert_approx_eq!(sensor.threshold_high.unwrap(), high_threshold);
    }

    #[test]
    fn test_humidity_hysteresis() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let hyst = 5.0;
        let result = sensor.set_relative_humidity_threshold_hysteresis(hyst);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.hysteresis.unwrap(), hyst);
    }

    #[test]
    fn test_humidity_hysteresis_mut_ref() {
        let mut sensor = MockHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let hyst = 3.0;
        let result = mut_ref.set_relative_humidity_threshold_hysteresis(hyst);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.hysteresis.unwrap(), hyst);
    }
}
use crate::decl_threshold_traits;
