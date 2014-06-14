use std::io::Reader;
use std::io::Writer;
use std::str::{from_utf8, from_utf8_owned};
use std::io::net::ip::SocketAddr;
use std::collections::Map;
use std::collections::HashMap;
use std::io::net::tcp::TcpStream;

use std::io::net::ip::{IpAddr};
use std::io::net::tcp::{TcpAcceptor, TcpListener};

use time;
use enums::*;


pub struct Server {
	addr: IpAddr,
	port: u16,
	src_bytes: u64,
	id: u64,
	str_addr: String,
	str_port: String,
	acceptor: TcpAcceptor
}

impl Server {
	fn new (address : &String, port: &String) -> Result <Server, ParseFailure>{
		let iPort = parse_int (port);
	}
}
