use std::io::Reader;
use std::io::Writer;
use std::str::{from_utf8, from_utf8_owned};
use std::io::net::ip::SocketAddr;
use std::collections::Map;
use std::collections::HashMap;
use std::io::net::tcp::TcpStream;

use time;


pub struct Client {
	peer: SocketAddr,
	dst_bytes: u64,
	src_bytes: u64,
	id: u64,
}

impl Client {

}
