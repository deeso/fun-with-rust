#![crate_id = "simple_protocol#0.10"]
#![crate_type = "lib"]
#![license = "BSD"]
#![comment = "simple protocol implementation."]

#![deny(non_camel_case_types)]
#![feature(macro_rules)]
#![feature(globs)]

extern crate time;
extern crate collections;
extern crate serialize;

pub use enums::*;
pub use client::Client;
pub use server::Server;
pub use parser::Parser;


mod parser;
mod enums;
mod client;
mod server;
mod protocol;