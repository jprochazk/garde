//! IP validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(ip)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Ip`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(ip)]` rule.
//!
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use std::fmt::Display;

use crate::error::Error;

pub fn apply<T: Ip>(v: &T, (kind,): (IpKind,)) -> Result<(), Error> {
    if v.validate_ip(kind).is_err() {
        return Err(Error::new(format!("not a valid {kind} address")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support IP validation",
        label = "This type does not support IP validation",
    )
)]
pub trait Ip {
    type Error: Display;

    fn validate_ip(&self, kind: IpKind) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy)]
pub enum IpKind {
    Any,
    V4,
    V6,
}

impl Display for IpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpKind::Any => write!(f, "IP"),
            IpKind::V4 => write!(f, "IPv4"),
            IpKind::V6 => write!(f, "IPv6"),
        }
    }
}

impl<T: AsRef<str>> Ip for T {
    type Error = std::net::AddrParseError;

    fn validate_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        let v = self.as_ref();
        match kind {
            IpKind::Any => {
                let _ = v.parse::<std::net::IpAddr>()?;
            }
            IpKind::V4 => {
                let _ = v.parse::<std::net::Ipv4Addr>()?;
            }
            IpKind::V6 => {
                let _ = v.parse::<std::net::Ipv6Addr>()?;
            }
        };
        Ok(())
    }
}
