#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub mod humidity;
pub mod sensor;
pub mod temperature;

#[doc(hidden)]
pub mod __private {
    pub use paste::paste;
}
