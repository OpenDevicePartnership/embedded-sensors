//! Async Sensor API
//!
//! This module contains traits generic to all sensors.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

pub use embedded_sensors_hal::sensor::{Error, ErrorKind, ErrorType};

/// Generates a threshold trait for the specified sensor type.
#[macro_export]
macro_rules! decl_threshold {
    ($SensorName:ident, $SensorTrait:ident, $SampleType:ty, $unit:expr) => {
        paste::paste! {
            #[doc = concat!(" Asynchronously set ", stringify!($SensorName), " thresholds.")]
            pub trait [<$SensorName Threshold>]: $SensorTrait {
                #[doc = concat!(" Set lower ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                async fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;

                #[doc = concat!(" Set upper ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                async fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;
            }

            impl<T: [<$SensorName Threshold>] + ?Sized> [<$SensorName Threshold>] for &mut T {
                async fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_low>](self, threshold).await
                }

                async fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_high>](self, threshold).await
                }
            }
        }
    };
}
