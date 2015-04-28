// Language Spec with all commands can be found here.
// https://www.zebra.com/content/dam/zebra/manuals/en-us/printer/epl2-pm-en.pdf
// Only a subset of all available commands are implemented.

// Supported commands:

#[derive(Debug)]
pub enum Rotation {
    NoRotation,
    Degrees90,
    Degrees180,
    Degrees270,
}

// Full font specification can be found on page 44 of the Zebra spec, but we
// will not support Soft Font or numeric-only fonts and thus only allow values 1
// through 5 for font selection
#[derive(Debug)]
pub enum Font {
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
}

// Only accepts 1-6, 8
// This struct exists to make parsing clearer
#[derive(Debug)]
pub struct HorizontalMultiplier {
    pub multiplier: i32,
}

// Only accepts 1-9
// This struct exists to make parsing clearer
#[derive(Debug)]
pub struct VerticalMultiplier {
    pub multiplier: i32,
}

#[derive(Debug)]
pub enum ReverseImage {
    N, // normal
    R, // reverse image
}

#[derive(Debug)]
pub struct AsciiText { // A
    pub h_start: i32,
    pub v_start: i32,
    pub rotation: Rotation,
    pub font_selection: Font,
    pub h_mult: HorizontalMultiplier,
    pub v_mult: VerticalMultiplier,
    pub reverse: ReverseImage,
    pub data: String, // Escaped string, "\"Company\"" should be parsed to store "Company" in the str
}

// There are tons of barcode types supported by the printer, see page 53 of spec.
// We only support Code128 for now
pub enum BarCodeStandardSelection {
    Code128Auto,
    Code128A,
    Code128B,
    Code128C,
}
pub struct BarCodeStandard { // B
    h_start: i32,
    v_start: i32,
    rotation: Rotation,
    bar_code: BarCodeStandardSelection,
    narrow_width: i32,
    wide_width: i32, // Only accepts 2-30
    bar_code_height: i32,
    print_human_readable: bool, // parsed with B = true, N = false
    data: String, // Escaped string
}

// EPL2 officially supports
// Aztec, Aztec Mesa, Data Matrix, MaxiCode, PDF417 and QR Code
// For now we only support PDF417
pub enum BarCode2DSelection {
    PDF417,
}

pub struct BarCode2D { // b
    h_start: i32,
    v_start: i32,
    bar_code: BarCode2DSelection,
    max_width: i32,
    max_height: i32,
    error_correction: i32,
    data_compression: i32, // Will be ignored
    print_human_readable: String, // format: xxx,yyy,mm - will be ignored
    origin_point: i32, // Should really be a bit, 0 or 1
    module_width: i32,
    bar_height: i32,
    max_rows: i32,
    max_cols: i32,
    truncated: bool,
    rotation: Rotation,
    data: String, // ASCII data or binary data bytes
}
pub struct Density { // D
    setting: i32, // Accepts 0-15
}
// Graphics has multiple possible subcommands, G,I,K,M and W,
// but only W is supported so that's what we're implementing here
pub struct Graphics { // G
    h_start: i32,
    v_start: i32,
    graphic_width: i32, // Width of graphic in bytes. Eight (8) dots = one (1) byte of data.
    graphic_length: i32, // Length of graphic in dots (or print lines)
    data: Vec<u8>, // Raw binary data
}

// EPL2 officially supports 8-bit and 7-bit data but we only handle 8-bit.
pub enum BitSupport {
    Bit8,
}

// We only support 8-bit encoding
pub enum Encoding {
    DOS437, // 0
    DOS850, // 1
    DOS852, // 2
    DOS860, // 3
    DOS863, // 4
    DOS865, // 5
    DOS857, // 6
    DOS861, // 7
    DOS862, // 8
    DOS855, // 9
    DOS866, // 10
    DOS737, // 11
    DOS851, // 12
    DOS869, // 13
    Windows1252, // A
    Windows1250, // B
    Windows1251, // C
    Windows1253, // D
    Windows1254, // E
    Windows1255, // F
}

pub enum KDUCountryCode {
    USA,         // 001
    Canada,      // 002
    LatinAm,     // 003
    SAfrica,     // 027
    Netherlands, // 031
    Belgium,     // 032
    France,      // 033
    Spain,       // 034
    Italy,       // 039
    Swizerland,  // 041
    UK,          // 044
    Denmark,     // 045
    Sweden,      // 046
    Norway,      // 047
    Germany,     // 049
    Portugal,    // 351
    Finland,     // 358
}

pub struct CharacterSetSelection { // I
    number_of_bits: BitSupport,
    language_support: Encoding,
    country_code: KDUCountryCode,
}


// Currently only support LO, Line Draw Black, so that's what this command is for.
pub struct LineDraw { // L
    h_start: i32,
    v_start: i32,
    h_length: i32,
    v_length: i32,
}

pub struct ClearImageBuffer; // N, "singleton command"

pub struct Print { // P
    nmbr_of_sets: i32, // 1 to 65535
    nmbr_of_copies: Option<i32>, // 1 to 65535
}

pub struct SetFormWidth { // q
    label_width: i32,
}

pub struct SetFormLength { // Q
    label_length: u16,
    gap_length: u16,
    offset_length: u16,
}

pub struct SpeedSelect { // S
    speed: u8, // What this means depends on printer-model so won't have any effect in this renderer
}

pub struct PrintConfiguration; // U, we parse this command so as not to choke on it, but ignore everything it does

pub struct WindowsMode { // W
    enabled: bool, // Y = true, N = false
}

pub struct BoxDraw { // X
    h_start: i32,
    v_start: i32,
    line_thickness: i32,
    h_end: i32,
    v_end: i32,
}

pub struct PrintDirection { // Z
    orientation: char, // T = printing from top, B = printing from bottom, not used
}
