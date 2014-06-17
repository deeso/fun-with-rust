use std::io::net::ip::SocketAddr;
use std::io::net::get_host_addresses;
use std::io::net::tcp::TcpStream;
use std::from_str::from_str;
use time::{Timespec, get_time};

#![crate_id = "simpleprotocol#0.11.0-pre"]
pub struct SimplePacket {
	len: u16,
	id: u16,
	timestamp: u32,
	msg_type: u8,
	msg: u8,
	msg_data: Vec<u8>
}


impl SimplePacket {
	fn new (id: u16, msg_type: u8, msg: u8, msg_data: Vec<u8>) -> SimplePacket{
		SimplePacket {
			len : 8 as u16,
			id : id,
			msg_type : msg_type,
			msg : msg,
			msg_data : msg_data.clone(),
			timestamp : (get_time()).sec as u32
		}
	}
}

