extern crate parser_combinators;

use std::borrow::{Cow};

use self::parser_combinators::{skip_many1, any_char, not_followed_by, crlf, between, spaces, many, many1, parser, sep_by, satisfy,
    Parser, ParserExt, ParseResult, ParseError, unexpected};
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
    let data = many(not_followed_by(parser(crlf)));
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



//command_separator: [u8] = [10, 0]
//parameter_separator: [u8] = [10, (byte)',']
//
//bytes: [u8]
//mut position: i32 // Might overflow with really, _really_ large EPL files
//
//// Indirectly recursive
//fn read_top() -> Option<[u8]> {
//    if (position >= bytes.length)
//        return None;
//    if (utility::is_command(bytes, position, commands::DIRECT_GRAPHICS_WRITE))
//        return Some(read_direct_graphics_write());
//    return Some(read_command(position));
//}
//
//fn read_direct_graphics_write() -> [u8] {
//    let start = position;
//    position += commands:DIRECT_GRAPHICS_WRITE.length;
//    let x = read_parameter_int();
//    let y = read_parameter_int();
//    let stride = read_parameter_int();
//    let height = read_parameter_int();
//    let bytes = stride * height;
//    position += bytes;
//    let command = read_command(start);
//
//    return command;
//}
//
//fn read_parameter_int() -> i32 {
//    let parameter = read_parameter_string(encoding::ASCII);
//    let value = int.Parse(parameter); // TODO: How to parse int?
//    return value;
//}
//
//fn read_parameter_string(encoding: Encoding) -> str {
//    let parameter = read_parameter();
//    let value = encoding.get_string(parameter);
//    return value;
//}
//
//fn read_command(start: int) -> [u8] {
//    let command = read(start, command_separator, true);
//    return command;
//}
//
//fn read(start: i32, breakOn: [u8], includeBreak: bool) -> [u8] {
//    for (; position < bytes.length; position++) {
//        if (breakOn.contains(bytes[position])) {
//            position++;
//            break;
//        }
//    }
//
//    let array = new array of length [position - start - (includeBreak ? 0 : 1)];
//    copy from bytes[start] into array[0] array.length bytes;
//
//    return array;
//}
//
//// todo: figure out if it's possible to return iterators in rust
//fn all_commands() -> [[u8]] {
//    position = 0;
//    let command: [u8];
//    while ((command = read()) != null)
//        yield return command;
//    yield break;
//}
