//! Async Temperature Sensor API
//!
//! This API provides generic methods for interfacing with temperature sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the TemperatureSensor and TemperatureThresholdWait traits for a temperature sensor.
//!
//! ```
//! use embedded_sensors_hal_async::sensor;
//! use embedded_sensors_hal_async::temperature::{
//!     DegreesCelsius, TemperatureHysteresis, TemperatureSensor,
//!     TemperatureThresholdSet, TemperatureThresholdWait,
//! };
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
//!     async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//!
//! impl TemperatureThresholdSet for MyTempSensor {
//!     async fn set_temperature_threshold_low(
//!         &mut self,
//!         threshold: DegreesCelsius
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold low register of sensor...
//!         Ok(())
//!     }
//!
//!     async fn set_temperature_threshold_high(
//!         &mut self,
//!         threshold: DegreesCelsius
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold high register of sensor...
//!         Ok(())
//!     }
//! }
//!
//! impl TemperatureThresholdWait for MyTempSensor {
//!     async fn wait_for_temperature_threshold(
//!         &mut self
//!     ) -> Result<DegreesCelsius, Self::Error> {
//!         // Await threshold alert (e.g. await GPIO level change on ALERT pin)...
//!         // Then return current temperature so caller can determine which threshold was crossed
//!         self.temperature().await
//!     }
//! }
//!
//! impl TemperatureHysteresis for MyTempSensor {
//!     async fn set_temperature_threshold_hysteresis(
//!         &mut self,
//!         hysteresis: DegreesCelsius
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold hysteresis register of sensor...
//!         Ok(())
//!     }
//! }
//! ```

use crate::decl_threshold_traits;
use crate::sensor::ErrorType;
pub use embedded_sensors_hal::temperature::DegreesCelsius;

/// Async Temperature Sensor methods.
pub trait TemperatureSensor: ErrorType {
    /// Returns a temperature sample in degrees Celsius.
    async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error>;
}

impl<T: TemperatureSensor + ?Sized> TemperatureSensor for &mut T {
    #[inline]
    async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
        T::temperature(self).await
    }
}

decl_threshold_traits!(
    Temperature,
    TemperatureSensor,
    DegreesCelsius,
    "degrees Celsius"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::{Error, ErrorKind};

    // Mock test values
    const TEST_TEMP_1: DegreesCelsius = 25.0;
    const TEST_TEMP_2: DegreesCelsius = 30.5;
    const TEST_THRESHOLD_LOW: DegreesCelsius = 15.0;
    const TEST_THRESHOLD_HIGH: DegreesCelsius = 35.0;
    const INITIAL_THRESHOLD: DegreesCelsius = 0.0;

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct MockAsyncTempSensor {
        value: DegreesCelsius,
        threshold_low: DegreesCelsius,
        threshold_high: DegreesCelsius,
    }

    impl crate::sensor::ErrorType for MockAsyncTempSensor {
        type Error = MockError;
    }

    impl TemperatureSensor for MockAsyncTempSensor {
        async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
            Ok(self.value)
        }
    }

    impl TemperatureThresholdSet for MockAsyncTempSensor {
        async fn set_temperature_threshold_low(
            &mut self,
            threshold: DegreesCelsius,
        ) -> Result<(), Self::Error> {
            self.threshold_low = threshold;
            Ok(())
        }

        async fn set_temperature_threshold_high(
            &mut self,
            threshold: DegreesCelsius,
        ) -> Result<(), Self::Error> {
            self.threshold_high = threshold;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_async_temperature_sensor_trait() {
        let mut sensor = MockAsyncTempSensor {
            value: TEST_TEMP_1,
            threshold_low: INITIAL_THRESHOLD,
            threshold_high: INITIAL_THRESHOLD,
        };
        let result = sensor.temperature().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_TEMP_1);
    }

    #[tokio::test]
    async fn test_async_temperature_sensor_trait_mut_ref() {
        let mut sensor = MockAsyncTempSensor {
            value: TEST_TEMP_2,
            threshold_low: INITIAL_THRESHOLD,
            threshold_high: INITIAL_THRESHOLD,
        };
        let mut_ref = &mut sensor;
        let result = mut_ref.temperature().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_TEMP_2);
    }

    #[tokio::test]
    async fn test_async_temperature_threshold_set_trait() {
        let mut sensor = MockAsyncTempSensor {
            value: TEST_TEMP_1,
            threshold_low: INITIAL_THRESHOLD,
            threshold_high: INITIAL_THRESHOLD,
        };

        let result_low = sensor
            .set_temperature_threshold_low(TEST_THRESHOLD_LOW)
            .await;
        assert!(result_low.is_ok());
        assert_eq!(sensor.threshold_low, TEST_THRESHOLD_LOW);

        let result_high = sensor
            .set_temperature_threshold_high(TEST_THRESHOLD_HIGH)
            .await;
        assert!(result_high.is_ok());
        assert_eq!(sensor.threshold_high, TEST_THRESHOLD_HIGH);
    }

    #[tokio::test]
    async fn test_async_temperature_threshold_set_trait_mut_ref() {
        let mut sensor = MockAsyncTempSensor {
            value: TEST_TEMP_1,
            threshold_low: INITIAL_THRESHOLD,
            threshold_high: INITIAL_THRESHOLD,
        };
        let mut_ref = &mut sensor;

        let result = mut_ref
            .set_temperature_threshold_low(TEST_THRESHOLD_LOW)
            .await;
        assert!(result.is_ok());
        assert_eq!(sensor.threshold_low, TEST_THRESHOLD_LOW);
    }
}
