use crate::garden::vegetables::Asparagus;
// use std::collections::HashMap;
// use std::io;
use std::{collections::HashMap, io};  // use nested paths to clean up Large use Lists
// use std::cmp;
// use std::cmp::Ordering
use std::cmp::{self, Ordering};
use std::io::*; // glob operator
use std::fmt;
use std::io::Result as IOResult;  // new name with as keyword
pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("Hello, world!");

    let mut map = HashMap::new();
    map.insert(1, "2");
}

// using Result types from different parent modules

// fn function1() -> fmt::Result {
//
// }
//
// fn function2() -> IOResult<()> {
// }