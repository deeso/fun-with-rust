use std::io::{File, Open, Read};
use std::string::{String};
use std::path::Path;
use std::os;
use std::str;


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
	let contents : Vec<u8>;
	let mut sz : u64 = 0;
	match fread_result {
	    // The read was valid
	    Ok(x) => {
	    	println!("File was valid");
	    	contents = x.clone();
	    	sz = path.stat().unwrap().size;
	    },
	    // The read was invalid
	    Err(e) => {
	    //None => {
	    	println!("File was invalid");
	    	contents = Vec::new();
	    }
	}
	let mut s = format!("Filename: {:s} Size: {:x}", *file_name, sz);
	println!("{}",s);
	s = format!("Contents Len: {:x}", contents.len());
	println!("{}",s);
}