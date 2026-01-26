//! Blocking Sensor API
//!
//! This module contains error-handling and traits generic to all sensors.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

/// Sensor error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Sensor error kind.
    ///
    /// By using this method, Sensor errors freely defined by HAL implementations
    /// can be converted to a set of generic Sensor errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Sensor error kind.
///
/// This represents a common set of Sensor operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common Sensor errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the sensor.
    /// e.g. An I2C error occurs for a digital sensor or an ADC error occurs for an analog sensor.
    /// The original error may contain more information.
    Peripheral,
    /// The sensor is not yet ready to be sampled.
    NotReady,
    /// The sensor is currently saturated and sample may be invalid.
    Saturated,
    /// The sensor was configured with invalid input.
    InvalidInput,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Peripheral => write!(
                f,
                "An error occured on the underlying peripheral. The original error may contain more informaton"
            ),
            Self::NotReady => write!(f, "Sensor is not yet ready to be sampled"),
            Self::Saturated => write!(f, "Sensor is saturated thus samples may be invalid"),
            Self::InvalidInput => write!(f, "Sensor was configured with invalid input"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Sensor error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Generates threshold traits for the specified sensor type.
///
/// This macro creates a unified API for both blocking and async sensor thresholds.
/// When used with `blocking` mode, it generates `ThresholdSet` and `Hysteresis` traits.
/// When used with `async` mode, it additionally generates `ThresholdWait` trait.
#[macro_export]
macro_rules! decl_threshold_traits {
    (blocking, $SensorName:ident, $SensorTrait:ident, $SampleType:ty, $unit:expr) => {
        paste::paste! {
            #[doc = concat!(" Set ", stringify!($SensorName), " thresholds.")]
            pub trait [<$SensorName ThresholdSet>]: $SensorTrait {
                #[doc = concat!(" Set lower ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;

                #[doc = concat!(" Set upper ", stringify!($SensorName), " threshold (in ", $unit, ").")]
                fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;
            }

            #[doc = concat!(" Set ", stringify!($SensorName), " threshold hysteresis.")]
            pub trait [<$SensorName Hysteresis>]: [<$SensorName ThresholdSet>] {
                #[doc = concat!(" Set ", stringify!($SensorName), " threshold hysteresis (in ", $unit, ").")]
                fn [<set_ $SensorName:snake _threshold_hysteresis>](&mut self, hysteresis: $SampleType) -> Result<(), Self::Error>;
            }

            impl<T: [<$SensorName ThresholdSet>] + ?Sized> [<$SensorName ThresholdSet>] for &mut T {
                fn [<set_ $SensorName:snake _threshold_low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_low>](self, threshold)
                }

                fn [<set_ $SensorName:snake _threshold_high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_high>](self, threshold)
                }
            }

            impl<T: [<$SensorName Hysteresis>] + ?Sized> [<$SensorName Hysteresis>] for &mut T {
                fn [<set_ $SensorName:snake _threshold_hysteresis>](&mut self, hysteresis: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_hysteresis>](self, hysteresis)
                }
            }
        }
    };

    (async, $SensorName:ident, $SensorTrait:ident, $SampleType:ty, $unit:expr) => {
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

            #[doc = concat!(" Asynchronously set ", stringify!($SensorName), " threshold hysteresis.")]
            pub trait [<$SensorName Hysteresis>]: [<$SensorName ThresholdSet>] {
                #[doc = concat!(" Set ", stringify!($SensorName), " threshold hysteresis (in ", $unit, ").")]
                async fn [<set_ $SensorName:snake _threshold_hysteresis>](&mut self, hysteresis: $SampleType) -> Result<(), Self::Error>;
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

            impl<T: [<$SensorName Hysteresis>] + ?Sized> [<$SensorName Hysteresis>] for &mut T {
                async fn [<set_ $SensorName:snake _threshold_hysteresis>](&mut self, hysteresis: $SampleType) -> Result<(), Self::Error> {
                    T::[<set_ $SensorName:snake _threshold_hysteresis>](self, hysteresis).await
                }
            }
        }
    };
}
