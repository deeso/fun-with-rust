#![crate_id = "r2extractor#0.1"]
#![crate_type = "lib"]

extern crate sync;
extern crate term;

use std::fmt;
use std::cell::RefCell;
use std::collections::{Deque,DList,HashMap};
use std::comm;
use std::mem;
use std::owned::Box;
use std::rc::Rc;
use std::task;
use std::vec::Vec;
use sync::Arc;
use std::rand::{task_rng,Rng};

pub mod error;


