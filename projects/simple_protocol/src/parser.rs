use std::str;

use std::io::Reader;
use std::str::from_utf8;

pub struct Parser<T> {
	priv iter: T,
}

