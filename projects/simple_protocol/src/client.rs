use std::io::Reader;
use std::io::Writer;
use std::vec::bytes::push_bytes;
use std::str::{from_utf8, from_utf8_owned};
use std::io::net::ip::SocketAddr;
use std::container::Map;
use collections::HashMap;
use std::io::net::tcp::TcpStream;

use time;


pub struct Client {
	priv peer: SocketAddr,
	priv dst_bytes: u64,
	priv src_bytes: u64,
	priv id: u64,
}

impl Client {

}
