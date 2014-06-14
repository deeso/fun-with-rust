use std::io::net::ip::SocketAddr;
use std::io::net::get_host_addresses;
use std::io::net::tcp::TcpStream;
use std::from_str::from_str;



struct Message {
	src: u64,
	dst: u64,
	id : u32,
	timestamp : u32,
	data : Vec<u8>,
}


impl Message {
	pub fn get_src (&self) -> u64{
		self.src
	}

	pub fn get_dst (&self) -> u64{
		self.dst
	}

	pub fn get_src (&self) -> u64{
		self.src
	}
}

