extern crate getopts;
use getopts::{optflag,getopts,OptGroup, usage, Matches};
use std::io::{File, Open, Read};
use std::path::Path;
use std::io::{Command};
use std::string::{String};
use std::os;
use std::str;


fn tell_me_file_size (path : &Path) {
	let valid_file = path.stat().is_ok() && path.is_file();

	if !valid_file {
		println!("File is not valid.");
		return;
	}

	let mut file = match File::open_mode(path, Open, Read) {
		Ok(f) => f,
		Err(e) => fail!("file error: {}", e),
	};


	let fread_result = file.read_to_end();
	// the match statement below could have been moved here
	// after the contents
	let contents : Vec<u8>;
	let mut sz : u64;
	match fread_result {
		// Read was successful so set the contents and file size.
		Ok(x) => {
			contents = x.clone();
			// hacky way of getting the size.
			// note contents sz could be obtained
			// using contents.len()
			sz = path.stat().unwrap().size;
			if sz != (contents.len() as u64) {
				println!("Uh-oh, the file lengths don't match invalid.");
			}
		},
		// The read was invalid so the contents are set 
		// to a new empty vector.
		Err(e) => {
			println!("Failed to read the file: {}", e);
			contents = Vec::new();
			sz = contents.len() as u64;
		}
	}
	//  print the filename and size in a formatted string
	let filename = path.as_str().unwrap();
	println!("Filename: {:s} Size: {:x}", filename, sz);
}

fn call_util_program (program : &str, filename : &str) -> String {
	// spawn a new process and use a match expression to determine if it was created
	let mut process =  match  Command::new(program).arg("-s").arg(filename).spawn()  {
		/*
		 	if it was created, the process is extracted from the IOResult, and assiged 
			to the process variable
		*/
		Ok(p) => p,
		// otherwise fail with the following message.
		Err(e) => fail!("failed to execute process: {}", e),
	};
	let output = match process.stdout.get_mut_ref().read_to_end() {
	  /*
		use a match expression to get the Vec<u8> out of the IOResult,
		if the result is Ok, then convert the Vec<u8> into a String.
		from_utf8_lossy is used to replace invalid utf8 chars, but it is
		lazy and will not create an "owned" string if not necessary.

		Note: as_slice is used to get the char buffer out of the Vec<u8>
	  */
		Ok(r) => str::from_utf8_lossy(r.as_slice ()).into_string(),
		// fail and print the message
		Err(e) => fail!("failed to execute process: {}", e),
	};
	return output;
}

fn print_usage(program : String, optgroups: &[OptGroup]) {
	/*

	print a generated usage statement for this program
		1) create the formated usage statement with the program name
		2) use the str array of the resulting usage format and array of
			options from the OptGroup to create the help statement.
	*/
	let generated_usage = usage( format!("Usage: {}", program).as_slice (), optgroups.as_slice());
	println!("{}", generated_usage);
}

fn get_program_opts () -> Vec<getopts::OptGroup>{
	// Create an array of options that will be used by 
	// this utility
	let opts /*-> Vec<getopts::OptGroup	>*/ = vec![
		optflag("d", "", "REQUIRED: dump symbols"),
		optflag("h", "help", "print this help menu")
	];
	return opts;
}

fn get_filename (matches : &mut getopts::Matches ) -> String{
	// get the last free argument from the Options Matches
	// free means the argument is not bound to any flags or
	// program options.
	if matches.free.len() <= 0 {
		fail!("No file specified.");
	}
	return matches.free.pop().unwrap();
}

fn main() {
	// get the program arguments, the program name, and the options.
	let args = os::args();
	let program = args.get(0).clone();
	let opts = get_program_opts ();

	// bind the program options to arguments and then check
	// and set parameters in the program
	let mut matches = match getopts (args.as_slice(), opts.as_slice()) {
		Ok (m) => {
			if m.opt_present ("h") {
				print_usage (program, opts.as_slice() );
				return;
			} else if !m.opt_present ("d") {
				// this parameter was not set, so print help and fail
				print_usage (program, opts.as_slice() );
				fail!("Error: need to specify the -d flag") ;
			}
			// set OptionMatches extracted from Result to the matches variable
			m
		}
		Err(f) => {
			fail!("Failed: {}", f);
		}
	};
	//let filename = matches.free.pop().unwrap();
	let util = "/bin/ls";
	// &mut is required since the  Matches will be modified when the
	// last free argument is pulled out of the free Vector
	let filename = get_filename (&mut matches);
	let path = Path::new(filename.as_bytes());
	tell_me_file_size (&path);

	println!("{} will execute {} with the -s flag.",util, filename);
	let output = call_util_program (util, filename.as_slice());
	println!("{}", output);
}
