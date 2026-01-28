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
use crate::decl_threshold_traits;

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

// This macro generates the following blocking threshold traits:
//
// pub trait TemperatureThresholdSet: TemperatureSensor {
//     fn set_temperature_threshold_low(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error>;
//     fn set_temperature_threshold_high(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error>;
// }
//
// pub trait TemperatureHysteresis: TemperatureThresholdSet {
//     fn set_temperature_threshold_hysteresis(&mut self, hysteresis: DegreesCelsius) -> Result<(), Self::Error>;
// }
decl_threshold_traits!(
    blocking,
    Temperature,
    TemperatureSensor,
    DegreesCelsius,
    "degrees Celsius"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::{Error, ErrorKind};
    use assert_approx_eq::assert_approx_eq;

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
        threshold_low: Option<DegreesCelsius>,
        threshold_high: Option<DegreesCelsius>,
        hysteresis: Option<DegreesCelsius>,
    }

    impl crate::sensor::ErrorType for MockTempSensor {
        type Error = MockError;
    }

    impl TemperatureSensor for MockTempSensor {
        fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
            Ok(self.value)
        }
    }

    impl TemperatureThresholdSet for MockTempSensor {
        fn set_temperature_threshold_low(
            &mut self,
            threshold: DegreesCelsius,
        ) -> Result<(), Self::Error> {
            self.threshold_low = Some(threshold);
            Ok(())
        }

        fn set_temperature_threshold_high(
            &mut self,
            threshold: DegreesCelsius,
        ) -> Result<(), Self::Error> {
            self.threshold_high = Some(threshold);
            Ok(())
        }
    }

    impl TemperatureHysteresis for MockTempSensor {
        fn set_temperature_threshold_hysteresis(
            &mut self,
            hysteresis: DegreesCelsius,
        ) -> Result<(), Self::Error> {
            self.hysteresis = Some(hysteresis);
            Ok(())
        }
    }

    #[test]
    fn test_temperature_sensor_trait() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let result = sensor.temperature();
        assert!(result.is_ok());
        assert_approx_eq!(result.unwrap(), TEST_TEMP);
    }

    #[test]
    fn test_temperature_sensor_trait_mut_ref() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let result = mut_ref.temperature();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_approx_eq!(value, TEST_TEMP);
    }

    #[test]
    fn test_temperature_threshold_set_low() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let threshold = 20.0;
        let result = sensor.set_temperature_threshold_low(threshold);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.threshold_low.unwrap(), threshold);
    }

    #[test]
    fn test_temperature_threshold_set_high() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let threshold = 30.0;
        let result = sensor.set_temperature_threshold_high(threshold);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.threshold_high.unwrap(), threshold);
    }

    #[test]
    fn test_temperature_threshold_set_mut_ref() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let low_threshold = 15.0;
        let high_threshold = 35.0;

        let result_low = mut_ref.set_temperature_threshold_low(low_threshold);
        assert!(result_low.is_ok());

        let result_high = mut_ref.set_temperature_threshold_high(high_threshold);
        assert!(result_high.is_ok());

        assert_approx_eq!(sensor.threshold_low.unwrap(), low_threshold);
        assert_approx_eq!(sensor.threshold_high.unwrap(), high_threshold);
    }

    #[test]
    fn test_temperature_hysteresis() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let hyst = 2.0;
        let result = sensor.set_temperature_threshold_hysteresis(hyst);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.hysteresis.unwrap(), hyst);
    }

    #[test]
    fn test_temperature_hysteresis_mut_ref() {
        let mut sensor = MockTempSensor {
            value: TEST_TEMP,
            threshold_low: None,
            threshold_high: None,
            hysteresis: None,
        };
        let mut_ref = &mut sensor;
        let hyst = 1.5;
        let result = mut_ref.set_temperature_threshold_hysteresis(hyst);
        assert!(result.is_ok());
        assert_approx_eq!(sensor.hysteresis.unwrap(), hyst);
    }
}
