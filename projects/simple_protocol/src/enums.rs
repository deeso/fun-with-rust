use std::str::from_utf8_owned;

#[deriving(Clone, Eq)]
pub enum ConnectFailure {
	HostNotFound,
	ConnectionRefused,
}