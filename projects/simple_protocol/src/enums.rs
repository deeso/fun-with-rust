use std::str::from_utf8_owned;

#[deriving(Clone, PartialEq, Eq)]
pub enum ConnectionFailure {
	HostNotFound,
	ConnectionRefused,
}