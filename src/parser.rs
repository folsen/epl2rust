extern crate parser_combinators;

use std::borrow::{Cow};

use self::parser_combinators::*;
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
    if i >= 1 && i <= 6 || i == 8 {
        Ok((commands::HorizontalMultiplier{multiplier: i}, input))
    } else {
        let msg = Cow::Owned(String::from("Invalid argument for Horizontal Multiplier"));
        Err(Consumed::Empty(ParseError::new(input.into_inner().position, Error::Message(msg))))
    }
}

fn parse_v_mult<I>(input: State<I>) -> ParseResult<commands::VerticalMultiplier, I>
    where I: Stream<Item=char> {
    let (i, input) = try!(parse_integer(input));
    if i >= 1 && i <= 9 {
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

fn escaped_char<I>(input: State<I>) -> ParseResult<char, I>
    where I: Stream<Item=char> {
    let (c, input) = try!(parser(any_char).parse_state(input));
    let mut back_slash_char = satisfy(|c| "\"\\/bfnrt".chars().find(|x| *x == c).is_some()).map(|c| {
        match c {
            '"' => '"',
            '\\' => '\\',
            '/' => '/',
            'b' => '\u{0008}',
            'f' => '\u{000c}',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            c => c//Should never happen
        }
    });
    match c {
        '\\' => input.combine(|input| back_slash_char.parse_state(input)),
        '"'  => unexpected("\"").parse_state(input.into_inner()).map(|_| unreachable!()),
        _    => Ok((c, input))
    }
}
fn escaped_string<I>(input: State<I>) -> ParseResult<String, I>
    where I: Stream<Item=char> {
    between(string("\""), string("\""), many(parser(escaped_char)))
        .parse_state(input)
}

// Parses something followed by a comma, discarding the comma
//fn lex<'a, P>(p: P) -> Skip<Self, P>
    //where P: Parser<Input=Self::Input> {
    //parser(p).skip(satisfy(|c| c == ','));
//}

fn lex<A, I>(p: &Fn(State<I>) -> ParseResult<A, I>, input: State<I>) -> ParseResult<A, I>
    where I: Stream<Item=char> {
    let mut lexed = parser(p).skip(satisfy(|c| c == ','));
    let (res, input) = try!(lexed.parse_state(input));
    return Ok((res, input))
}

pub fn parse_ascii_text<I>(input: State<I>) -> ParseResult<commands::AsciiText, I>
    where I: Stream<Item=char> {
    //let s = "A79,216,0,4,2,2,N,\"USPS\"";
    let mut command = satisfy(|c| c == 'A');

    let (_, input)        = try!(command.parse_state(input));
    let (h_start, input)  = try!(lex(&parse_integer, input.into_inner()));
    let (v_start, input)  = try!(lex(&parse_integer, input.into_inner()));
    let (rotation, input) = try!(lex(&parse_rotation, input.into_inner()));
    let (font, input)     = try!(lex(&parse_font, input.into_inner()));
    let (h_mult, input)   = try!(lex(&parse_h_mult, input.into_inner()));
    let (v_mult, input)   = try!(lex(&parse_v_mult, input.into_inner()));
    let (reverse, input)  = try!(lex(&parse_reverse, input.into_inner()));
    let (data, input)     = try!(escaped_string(input.into_inner()));

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

