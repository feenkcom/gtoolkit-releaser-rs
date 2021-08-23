#[macro_use]
extern crate lazy_static;
extern crate serde_derive;

mod error;
mod release;
mod version;

pub use error::{ReleaserError, Result};
pub use release::{GitHub, Release};
pub use version::{Version, VersionBump};
