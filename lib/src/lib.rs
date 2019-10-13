#[cfg(feature = "derive")]
extern crate soap_rs_derive;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use soap_rs_derive::*;

pub use minidom::{Children, Element};

mod display;
pub use display::DisplayAction;

mod envelope;
pub use envelope::Envelope;

mod error;
pub use error::Error;
