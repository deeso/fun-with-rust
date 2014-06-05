use std::io::{File, Open, Read};
use std::path::Path;
use std::os;
use std::str;

fn print_hex (bytes: &Vec<u8>) {

	println!("Len of bytes: {}", bytes.len());

}

fn main() {
	let args = os::args();
	let mut iter = args.iter().skip(1); // skip the program name
	let mut file_name : ~str = iter.next()

	println!("Filename: {}", file_name);
	//let path = &Path::new(file_name.as_slice());
	/*let file = match File::open_mode(path, Open, Read) {
		Ok(f) => f,
		Err(e) => fail!("file error: {}", e),
	};*/
	//let file = File::open(path);
	//let contents = &file.read_to_end().ok().unwrap_or(Vec::new());;
	//let sz = path.stat().unwrap().size;
	//print_hex (contents);
}