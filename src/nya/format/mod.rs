// forward all miau formats
pub use miau::format::*;

#[cfg(feature = "hocon")]
mod hocon;
#[cfg(feature = "hjson")]
mod hjson;
#[cfg(feature = "dhall")]
mod dhall;

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