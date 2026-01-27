//! Async Humidity Sensor API
//!
//! This API provides generic methods for interfacing with humidity sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the RelativeHumiditySensor
//! and RelativityHumidityThresholdWait traits for a humidity sensor.
//!
//! ```
//! use embedded_sensors_hal_async::sensor;
//! use embedded_sensors_hal_async::humidity::{
//!     Percentage, RelativeHumidityHysteresis, RelativeHumiditySensor,
//!     RelativeHumidityThresholdSet, RelativeHumidityThresholdWait,
//! };
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
//!     async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//!
//! impl RelativeHumidityThresholdSet for MyHumiditySensor {
//!     async fn set_relative_humidity_threshold_low(
//!         &mut self,
//!         threshold: Percentage
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold low register of sensor...
//!         Ok(())
//!     }
//!
//!     async fn set_relative_humidity_threshold_high(
//!         &mut self,
//!         threshold: Percentage
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold high register of sensor...
//!         Ok(())
//!     }
//! }
//!
//! impl RelativeHumidityThresholdWait for MyHumiditySensor {
//!     async fn wait_for_relative_humidity_threshold(
//!         &mut self,
//!     ) -> Result<Percentage, Self::Error> {
//!         // Await threshold alert (e.g. await GPIO level change on ALERT pin)...
//!         // Then return current relative humidity so caller can determine which threshold was crossed
//!         self.relative_humidity().await
//!     }
//! }
//!
//! impl RelativeHumidityHysteresis for MyHumiditySensor {
//!     async fn set_relative_humidity_threshold_hysteresis(
//!         &mut self,
//!         hysteresis: Percentage
//!     ) -> Result<(), Self::Error> {
//!         // Write value to threshold hysteresis register of sensor...
//!         Ok(())
//!     }
//! }
//! ```

use crate::decl_threshold_traits;
use crate::sensor::ErrorType;
pub use embedded_sensors_hal::humidity::Percentage;

/// Async Relative Humidity Sensor methods.
pub trait RelativeHumiditySensor: ErrorType {
    /// Returns a relative humidity (RH) sample as a percentage.
    async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error>;
}

impl<T: RelativeHumiditySensor + ?Sized> RelativeHumiditySensor for &mut T {
    #[inline]
    async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
        T::relative_humidity(self).await
    }
}

decl_threshold_traits!(
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

    // Mock test values
    const TEST_HUMIDITY: Percentage = 65.0;
    const TEST_THRESHOLD_LOW: Percentage = 30.0;
    const TEST_THRESHOLD_HIGH: Percentage = 80.0;
    const TEST_INITIAL_THRESHOLD: Percentage = 0.0;

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct MockAsyncHumiditySensor {
        value: Percentage,
        threshold_low: Percentage,
        threshold_high: Percentage,
    }

    impl crate::sensor::ErrorType for MockAsyncHumiditySensor {
        type Error = MockError;
    }

    impl RelativeHumiditySensor for MockAsyncHumiditySensor {
        async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
            Ok(self.value)
        }
    }

    impl RelativeHumidityThresholdSet for MockAsyncHumiditySensor {
        async fn set_relative_humidity_threshold_low(
            &mut self,
            threshold: Percentage,
        ) -> Result<(), Self::Error> {
            self.threshold_low = threshold;
            Ok(())
        }

        async fn set_relative_humidity_threshold_high(
            &mut self,
            threshold: Percentage,
        ) -> Result<(), Self::Error> {
            self.threshold_high = threshold;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_async_humidity_sensor_trait() {
        let mut sensor = MockAsyncHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: TEST_INITIAL_THRESHOLD,
            threshold_high: TEST_INITIAL_THRESHOLD,
        };
        let result = sensor.relative_humidity().await;
        assert!(result.is_ok());
        assert_approx_eq!(result.unwrap(), TEST_HUMIDITY);
    }

    #[tokio::test]
    async fn test_async_humidity_sensor_trait_mut_ref() {
        let mut sensor = MockAsyncHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: TEST_INITIAL_THRESHOLD,
            threshold_high: TEST_INITIAL_THRESHOLD,
        };
        let mut_ref = &mut sensor;
        let result = mut_ref.relative_humidity().await;
        assert!(result.is_ok());
        assert_approx_eq!(result.unwrap(), TEST_HUMIDITY);
    }

    #[tokio::test]
    async fn test_async_humidity_threshold_set_trait() {
        let mut sensor = MockAsyncHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: TEST_INITIAL_THRESHOLD,
            threshold_high: TEST_INITIAL_THRESHOLD,
        };

        let result_low = sensor
            .set_relative_humidity_threshold_low(TEST_THRESHOLD_LOW)
            .await;
        assert!(result_low.is_ok());
        assert_approx_eq!(sensor.threshold_low, TEST_THRESHOLD_LOW);

        let result_high = sensor
            .set_relative_humidity_threshold_high(TEST_THRESHOLD_HIGH)
            .await;
        assert!(result_high.is_ok());
        assert_approx_eq!(sensor.threshold_high, TEST_THRESHOLD_HIGH);
    }

    #[tokio::test]
    async fn test_async_humidity_threshold_set_trait_mut_ref() {
        let mut sensor = MockAsyncHumiditySensor {
            value: TEST_HUMIDITY,
            threshold_low: TEST_INITIAL_THRESHOLD,
            threshold_high: TEST_INITIAL_THRESHOLD,
        };

        {
            let mut_ref = &mut sensor;
            let result_low = mut_ref
                .set_relative_humidity_threshold_low(TEST_THRESHOLD_LOW)
                .await;
            assert!(result_low.is_ok());
        }

        assert_approx_eq!(sensor.threshold_low, TEST_THRESHOLD_LOW);

        {
            let mut_ref = &mut sensor;
            let result_high = mut_ref
                .set_relative_humidity_threshold_high(TEST_THRESHOLD_HIGH)
                .await;
            assert!(result_high.is_ok());
        }

        assert_approx_eq!(sensor.threshold_high, TEST_THRESHOLD_HIGH);
    }
}
