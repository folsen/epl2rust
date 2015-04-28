extern crate parser_combinators;
extern crate epl2img;

use std::io::prelude::*;
use std::fs::File;

use parser_combinators::{parser};
use parser_combinators::primitives::{Parser};
use epl2img::commands;
use epl2img::parser;

fn main() {
    let mut file = match File::open("test.txt") {
        Ok(f) => f,
        Err(_) => return println!("Couldn't open file"),
    };
    let mut s = String::new();
    file.read_to_string(&mut s);

    let mys = parser(parser::parse_ascii_text).parse(&*s);
    println!("{:?}", mys)
}
