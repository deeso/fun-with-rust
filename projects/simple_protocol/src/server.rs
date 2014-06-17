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
	addr: String,
	port: u16,
	acceptor: TcpAcceptor,
	clients : HashMap< Box<String>, TcpStream>
}

impl Server {

}
