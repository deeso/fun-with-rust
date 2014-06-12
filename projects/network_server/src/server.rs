extern crate getopts;
use getopts::{optflag,getopts,OptGroup, usage, reqopt, optopt};
use std::string::{String};
use std::os;


use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn parse_int (input: &String) -> u32 {
	let val = match from_str::<u32>(input.as_slice()) {
		Some(0) => 0,
		Some(x) => x,
		None => 0
	};
	return val;
}


fn create_listener (address: &String, port: &String) -> TcpListener {
	let iPort = parse_int (port);

	if iPort <= 0{
		fail!("Invalid port value: {}", port);
	}
	let listener = match TcpListener::bind(address.as_slice(), iPort as u16) {
		Ok(l) => l,
		Err(e) => fail!("Err: Unable to start the listener server: {}", e)
	};
	return listener;
}


fn get_program_opts () -> Vec<getopts::OptGroup>{
	// Create an array of options that will be used by 
	// this utility
	let opts /*-> Vec<getopts::OptGroup	>*/ = vec![
		reqopt("p", "port", "port to listen on", "PORT"),
		reqopt("a", "address", "interface to listen on", "ADDRESS"),
		optflag("h", "help", "print this help menu")
	];
	return opts;
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

fn handle_client (mut stream: TcpStream) {
	stream.write(bytes!("Hello\n\n"));
	stream.write(bytes!("Goodbye\n\n"));
}

fn run_server (mut listener : TcpListener) {
	let mut acceptor = listener.listen();
	for stream in acceptor.incoming() {
		match stream {
			Err(e) => { println!("connection failed: {}", e) }
			Ok(stream) => spawn(proc() {
				// connection succeeded
				handle_client(stream)
			})
		}
	}
	drop(acceptor);
}

fn main () {
	let args = os::args();
	let program = args.get(0).clone();
	let opts = get_program_opts ();

	// bind the program options to arguments and then check
	// and set parameters in the program
	let matches = match getopts (args.as_slice(), opts.as_slice()) {
		Ok (m) => {
			if m.opt_present ("h") {
				print_usage (program, opts.as_slice() );
				return;
			} else if !m.opt_present ("p") {
				// this parameter was not set, so print help and fail
				print_usage (program, opts.as_slice() );
				fail!("Error: need to specify a port to listen on.") ;
			}
			// set OptionMatches extracted from Result to the matches variable
			m
		}
		Err(f) => {
			fail!("Failed: {}", f);
		}
	};

	let port = match matches.opt_str("p") {
		Some (x) => x,
		None => fail!("Failed to read the port value.")
	};
	let address = if matches.opt_present("a") {
		matches.opt_str("a").unwrap()
	} else {
		let k = String::from_str("");
		println!("Address is Nil, settingo setting it too: {}", k);
		k
	};

	let listener = create_listener (&address, &port);
	run_server (listener)
}
