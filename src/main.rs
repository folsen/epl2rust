extern crate parser_combinators;
extern crate epl2img;

use parser_combinators::{parser};
use parser_combinators::primitives::{Parser};
use epl2img::commands;
use epl2img::parser;

fn main() {
    let s = "A79,216,0,4,2,2,N,\"USPS\"";
    let mys = parser(parser::parse_ascii_text).parse(s);
    println!("{:?}", mys)
}
