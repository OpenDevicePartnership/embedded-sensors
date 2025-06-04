//! Async Sensor API
//!
//! This module contains traits generic to all sensors.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

pub use embedded_sensors_hal::sensor::{Error, ErrorKind, ErrorType};

/// Generates threshold traits for the specified sensor type.
#[macro_export]
macro_rules! decl_threshold_traits {
    ($SensorName:ident, $SensorTrait:ident, $SampleType:ty, $unit:expr) => {
        paste::paste! {
            #[doc = concat!(" Asynchronously set ", stringify!($SensorName), " thresholds.")]
            pub trait [<$SensorName ThresholdSet>]: $SensorTrait {
                #[doc = concat!(" Set lower ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                async fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;

                #[doc = concat!(" Set upper ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                async fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;
            }

            #[doc = concat!(" Asynchronously wait for ", stringify!($SensorName), " measurements to exceed specified thresholds.")]
            pub trait [<$SensorName ThresholdWait>]: [<$SensorName ThresholdSet>] {
                #[doc = concat!(" Wait for ", stringify!($SensorName), " to be measured above or below the previously set high and low thresholds.")]
                #[doc = concat!(" Returns the measured ", stringify!($SensorName), " at time threshold is exceeded (in ", $unit, ").")]
                async fn [<wait_for_ $SensorName:snake _threshold>](&mut self) -> Result<$SampleType, Self::Error>;
            }

            impl<T: [<$SensorName ThresholdSet>] + ?Sized> [<$SensorName ThresholdSet>] for &mut T {
                async fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_low>](self, threshold).await
                }

                async fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_high>](self, threshold).await
                }
            }

            impl<T: [<$SensorName ThresholdWait>] + ?Sized> [<$SensorName ThresholdWait>] for &mut T {
                async fn [<wait_for_ $SensorName:snake _threshold>](&mut self) -> Result<$SampleType, Self::Error> {
                    T::[<wait_for_ $SensorName:snake _threshold>](self).await
                }
            }
        }
    };
}
