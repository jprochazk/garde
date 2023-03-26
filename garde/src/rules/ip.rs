use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;

pub fn apply<T: Ip>(v: &T, (kind,): (IpKind,)) -> Result<(), Error> {
    if v.try_parse_ip(kind).is_err() {
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

    fn try_parse_ip(&self, kind: IpKind) -> Result<(), Self::Error>;
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

fn check_str(s: &str, kind: IpKind) -> Result<(), std::net::AddrParseError> {
    match kind {
        IpKind::Any => std::net::IpAddr::from_str(s)?,
        IpKind::V4 => std::net::Ipv4Addr::from_str(s)?.into(),
        IpKind::V6 => std::net::Ipv6Addr::from_str(s)?.into(),
    };
    Ok(())
}

impl Ip for String {
    type Error = std::net::AddrParseError;

    fn try_parse_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        check_str(self.as_str(), kind)
    }
}
impl<'a> Ip for &'a str {
    type Error = std::net::AddrParseError;

    fn try_parse_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        check_str(self, kind)
    }
}
impl<'a> Ip for std::borrow::Cow<'a, str> {
    type Error = std::net::AddrParseError;

    fn try_parse_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        check_str(self.as_ref(), kind)
    }
}
