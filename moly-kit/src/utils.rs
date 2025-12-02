//! Internally used to hold utility modules but exposes some very helpful ones.

pub mod errors;
pub mod makepad;
pub(crate) mod platform;
pub(crate) mod scraping;
#[cfg(feature = "json")]
pub(crate) mod serde;
