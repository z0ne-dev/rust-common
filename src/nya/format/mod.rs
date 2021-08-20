// forward all miau formats
pub use miau::format::*;

#[cfg(feature = "hocon")]
pub mod hocon;
#[cfg(feature = "hjson")]
pub mod hjson;
#[cfg(feature = "dhall")]
pub mod dhall;

#[cfg(feature = "hocon")]
pub fn hocon() -> hocon::Hocon {
    hocon::Hocon::default()
}

#[cfg(feature = "hjson")]
pub fn hjson() -> hjson::Hjson {
    hjson::Hjson::default()
}

#[cfg(feature = "dhall")]
pub fn dhall() -> dhall::Dhall {
    dhall::Dhall::default()
}