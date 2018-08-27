#![feature(associated_type_defaults)]
#![feature(test)]
#![allow(unused_imports)]
#![warn(missing_docs, missing_debug_implementations)]
use std::collections::{btree_set, BTreeSet};
use std::default::Default;
use std::thread::sleep;
use std::time::Duration;
use test::{black_box, Bencher};

mod prime_generator;

fn main() {}
