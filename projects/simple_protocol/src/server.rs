use std::io::Reader;
use std::io::Writer;
use std::vec::bytes::push_bytes;
use std::str::{from_utf8, from_utf8_owned};
use std::io::net::ip::SocketAddr;
use std::container::Map;
use collections::HashMap;
use std::io::net::tcp::TcpStream;

use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

use time;
use enums::*;


pub struct Server {
	priv addr: IpAddr,
	priv port: u16,
	priv src_bytes: u64,
	priv id: u64,
	priv str_addr: String,
	priv str_port: String,
	priv acceptor: TcpAcceptor
}

impl Server {
	fn init_listener (address : &String, port: String) {

	}


}
