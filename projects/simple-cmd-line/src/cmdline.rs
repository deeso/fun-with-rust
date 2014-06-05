use std::io::{File, Open, Read};
use std::string::{String};
use std::path::Path;
use std::os;
use std::str;

fn print_hex (bytes: &Vec<u8>) {
	println!("Len of bytes: {}", bytes.len());
}

fn main() {
	let args = os::args();
	let mut iter = args.iter().skip(1); // skip the program name
	let mut file_name : &String = (iter.next().as_slice())[0];

	println!("Filename: {}", file_name);
	let path = Path::new(file_name.as_bytes());
	/*let file = match File::open_mode(path, Open, Read) {
		Ok(f) => f,
		Err(e) => fail!("file error: {}", e),
	};*/
	let mut file = File::open(&path);
	let fread_result = file.read_to_end();
	let contents : Option<Vec<u8>> = None;
	match fread_result {
	    // The read was valid
	    Ok(x) => {
	    	println!("File was valid");
	    	contents = x.clone();
	    },
	    // The read was invalid
	    Err(e) => {
	    	println!("File was invalid");
	    	contents = Vec::new();
	    }
	}
	let sz : u64 = path.stat().unwrap().size;
	let s = format!("Filename: {:s} Size: {:x}", *file_name, sz);
	let s = format!("Contents Len: {:x}", contents.len());
	println!("{}",s);
	//print_hex (contents);
}