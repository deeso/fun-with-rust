#![crate_id = "cmdline"]
#![crate_type = "bin"]
#![license = "BSD"]
#![comment = "Rust implementation for cmdline."]

#![deny(non_camel_case_types)]
#![feature(macro_rules)]
#![feature(globs)]

extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::strbuf::StrBuf;
use std::strbuf;
fn do_work(inp: &str, out: Option<StrBuf>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-o\t\tOutput");
    println!("-h --help\tUsage");
}

fn main() {
    let args: Vec<StrBuf> = os::args().iter()
                                      .map(|x| x.to_strbuf())
                                      .collect();

    let program = args.get(0).clone();

    let opts = [
        optopt("o", "", "set output file name", "NAME"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        (*matches.free.get(0)).clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };
    do_work(input.as_slice(), output);
}
