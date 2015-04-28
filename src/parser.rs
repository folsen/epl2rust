extern crate parser_combinators;

use std::borrow::{Cow};

use self::parser_combinators::*;
use self::parser_combinators::combinator::{Many};
use self::parser_combinators::primitives::{State, Stream, Error, Consumed};

use commands;

fn parse_rotation<I>(input: State<I>) -> ParseResult<commands::Rotation, I>
    where I: Stream<Item=char> {
    let (i, input) = try!(parse_integer(input));
    let msg = Cow::Owned(String::from("Invalid argument for Rotation"));
    match i {
        0 => Ok((commands::Rotation::NoRotation, input)),
        1 => Ok((commands::Rotation::Degrees90, input)),
        2 => Ok((commands::Rotation::Degrees180, input)),
        3 => Ok((commands::Rotation::Degrees270, input)),
        _ => Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg)))),
    }
}

fn parse_font<I>(input: State<I>) -> ParseResult<commands::Font, I>
    where I: Stream<Item=char> {
    let (i, input) = try!(parse_integer(input));
    let msg = Cow::Owned(String::from("Invalid argument for Font"));
    match i {
        1 => Ok((commands::Font::Size1, input)),
        2 => Ok((commands::Font::Size2, input)),
        3 => Ok((commands::Font::Size3, input)),
        4 => Ok((commands::Font::Size4, input)),
        5 => Ok((commands::Font::Size5, input)),
        _ => Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg)))),
    }
}

fn parse_h_mult<I>(input: State<I>) -> ParseResult<commands::HorizontalMultiplier, I>
    where I: Stream<Item=char> {
    let (i, input) = try!(parse_integer(input));
    if (i >= 1 && i <= 6 || i == 8) {
        Ok((commands::HorizontalMultiplier{multiplier: i}, input))
    } else {
        let msg = Cow::Owned(String::from("Invalid argument for Horizontal Multiplier"));
        Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg))))
    }
}

fn parse_v_mult<I>(input: State<I>) -> ParseResult<commands::VerticalMultiplier, I>
    where I: Stream<Item=char> {
    let (i, input) = try!(parse_integer(input));
    if (i >= 1 && i <= 9) {
        Ok((commands::VerticalMultiplier{multiplier: i}, input))
    } else {
        let msg = Cow::Owned(String::from("Invalid argument for Vertical Multiplier"));
        Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg))))
    }
}

fn parse_reverse<I>(input: State<I>) -> ParseResult<commands::ReverseImage, I>
    where I: Stream<Item=char> {
    let mut letter = satisfy(|c| c.is_alphabetic());
    let (l, input) = try!(letter.parse_state(input));

    let msg = Cow::Owned(String::from("Invalid argument for Reverse Image"));
    match l {
        'N' => Ok((commands::ReverseImage::N, input)),
        'R' => Ok((commands::ReverseImage::N, input)),
        _ => Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg)))),
    }
}

fn parse_integer<I>(input: State<I>) -> ParseResult<i32, I>
    where I: Stream<Item=char> {
    let mut integer = many1(satisfy(|c| c.is_numeric()));
    let (i, input) : (String, Consumed<State<I>>) = try!(integer.parse_state(input));
    Ok((i.parse::<i32>().unwrap(), input))
}

fn parse_string_data<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item=char> {
    let wo_escape = many(satisfy(|c| c != '\\' && c != '"'));
    let escape = many(string("\\\""));

    // TODO: Incomplete, crashes in a backslash in string
    let data = try(wo_escape).or(escape);

    //let data = many(choice([wo_escape, escape]));
    let mut data_string = between(satisfy(|c| c == '"'), satisfy(|c| c == '"'), data);
    data_string.parse_state(input)
}

pub fn parse_ascii_text<I>(input: State<I>) -> ParseResult<commands::AsciiText, I>
    where I: Stream<Item=char> {
    //let s = "A79,216,0,4,2,2,N,\"USPS\"";
    let mut command = satisfy(|c| c == 'A');
    let mut comma = satisfy(|c| c == ',');


    let (_, input)        = try!(command.parse_state(input));
    let (h_start, input)  = try!(parse_integer(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (v_start, input)  = try!(parse_integer(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (rotation, input) = try!(parse_rotation(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (font, input)     = try!(parse_font(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (h_mult, input)   = try!(parse_h_mult(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (v_mult, input)   = try!(parse_v_mult(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (reverse, input)  = try!(parse_reverse(input.into_inner()));
    let (_, input)        = try!(comma.parse_state(input.into_inner()));
    let (data, input)     = try!(parse_string_data(input.into_inner()));

    let at = commands::AsciiText {
        h_start: h_start,
        v_start: v_start,
        rotation: rotation,
        font_selection: font,
        h_mult: h_mult,
        v_mult: v_mult,
        reverse: reverse,
        data: data,
    };

    return Ok((at, input))
}

