use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};
use phf::phf_map;

#[derive(Clone, Debug)]
pub enum ExtendedChars {
    Partial,
    Byte(u8),
    Char(char),
    CharString(String),
    AltChar(char),
    CtrlChar(char),
    CtrlAltChar(char),
    Normal(SpecialChars),
    Shift(SpecialChars),
    Alt(SpecialChars),
    Ctrl(SpecialChars),
    ShiftAlt(SpecialChars),
    CtrlShift(SpecialChars),
    CtrlAlt(SpecialChars),
    CtrlShiftAlt(SpecialChars),
    ExtendedString(Vec<ExtendedChars>),
    Quit,
}

impl Display for ExtendedChars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Partial => { write!(f, "<Partial>") },
            Byte(b) => { write!(f, "<{:#X}>", b) },
            Char(c) => { write!(f, "{}", c) },
            CharString(s) => { write!(f, "{}", s) },
            CtrlChar(c) => { write!(f, "<Ctrl-{}>", c) },
            AltChar(c) => { write!(f, "<Alt-{}>", c) },
            CtrlAltChar(c) => { write!(f, "<Ctrl-Alt-{}>", c) },
            Normal(e) => { write!(f, "<{:?}>", e) },
            Shift(e) => { write!(f, "<Shift-{:?}>", e) },
            Alt(e) => { write!(f, "<Alt-{:?}>", e) },
            Ctrl(e) => { write!(f, "<Ctrl-{:?}>", e) },
            ShiftAlt(e) => { write!(f, "<Shift-Alt-{:?}>", e) },
            CtrlShift(e) => { write!(f, "<Ctrl-Shift-{:?}>", e) },
            CtrlAlt(e) => { write!(f, "<Ctrl-Alt-{:?}>", e) },
            CtrlShiftAlt(e) => { write!(f, "<Ctrl-Shift-Alt-{:?}>", e) },
            ExtendedString(es) => {
                for e in es.into_iter() {
                    write!(f, "{}", e)?;
                }
                Ok(())
            },
            Quit => { write!(f, "<Quit>") },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SpecialChars {
    Nul,
    Backspace,
    Tab,
    Escape,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

use SpecialChars::*;
use ExtendedChars::*;
pub static CHAR_CODES: phf::Map<&'static str, ExtendedChars> = phf_map! {
    "\x00" => Normal(Nul),
    "\x09" => Normal(Tab),
    "\x1b" => Partial,
    "\x1b\t" => Alt(Tab),
    "\x1bO" => Partial,
    "\x1bOP" => Normal(F1),
    "\x1bOQ" => Normal(F2),
    "\x1bOR" => Normal(F3),
    "\x1bOS" => Normal(F4),
    "\x1b[" => Partial,
    "\x1b[1" => Partial,
    "\x1b[15" => Partial,
    "\x1b[15;" => Partial,
    "\x1b[15;2" => Partial,
    "\x1b[15;2~" => Shift(F5),
    "\x1b[15;3" => Partial,
    "\x1b[15;3~" => Alt(F5),
    "\x1b[15;4" => Partial,
    "\x1b[15;4~" => ShiftAlt(F5),
    "\x1b[15;5" => Partial,
    "\x1b[15;5~" => Ctrl(F5),
    "\x1b[15;6" => Partial,
    "\x1b[15;6~" => CtrlShift(F5),
    "\x1b[15;7" => Partial,
    "\x1b[15;7~" => CtrlAlt(F5),
    "\x1b[15;8" => Partial,
    "\x1b[15;8~" => CtrlShiftAlt(F5),
    "\x1b[15~" => Normal(F5),
    "\x1b[17" => Partial,
    "\x1b[17;" => Partial,
    "\x1b[17;2" => Partial,
    "\x1b[17;2~" => Shift(F6),
    "\x1b[17;3" => Partial,
    "\x1b[17;3~" => Alt(F6),
    "\x1b[17;4" => Partial,
    "\x1b[17;4~" => ShiftAlt(F6),
    "\x1b[17;5" => Partial,
    "\x1b[17;5~" => Ctrl(F6),
    "\x1b[17;6" => Partial,
    "\x1b[17;6~" => CtrlShift(F6),
    "\x1b[17;7" => Partial,
    "\x1b[17;7~" => CtrlAlt(F6),
    "\x1b[17;8" => Partial,
    "\x1b[17;8~" => CtrlShiftAlt(F6),
    "\x1b[17~" => Normal(F6),
    "\x1b[18" => Partial,
    "\x1b[18;" => Partial,
    "\x1b[18;2" => Partial,
    "\x1b[18;2~" => Shift(F7),
    "\x1b[18;3" => Partial,
    "\x1b[18;3~" => Alt(F7),
    "\x1b[18;4" => Partial,
    "\x1b[18;4~" => ShiftAlt(F7),
    "\x1b[18;5" => Partial,
    "\x1b[18;5~" => Ctrl(F7),
    "\x1b[18;6" => Partial,
    "\x1b[18;6~" => CtrlShift(F7),
    "\x1b[18;7" => Partial,
    "\x1b[18;7~" => CtrlAlt(F7),
    "\x1b[18;8" => Partial,
    "\x1b[18;8~" => CtrlShiftAlt(F7),
    "\x1b[18~" => Normal(F7),
    "\x1b[19" => Partial,
    "\x1b[19;" => Partial,
    "\x1b[19;2" => Partial,
    "\x1b[19;2~" => Shift(F8),
    "\x1b[19;3" => Partial,
    "\x1b[19;3~" => Alt(F8),
    "\x1b[19;4" => Partial,
    "\x1b[19;4~" => ShiftAlt(F8),
    "\x1b[19;5" => Partial,
    "\x1b[19;5~" => Ctrl(F8),
    "\x1b[19;6" => Partial,
    "\x1b[19;6~" => CtrlShift(F8),
    "\x1b[19;7" => Partial,
    "\x1b[19;7~" => CtrlAlt(F8),
    "\x1b[19;8" => Partial,
    "\x1b[19;8~" => CtrlShiftAlt(F8),
    "\x1b[19~" => Normal(F8),
    "\x1b[1;" => Partial,
    "\x1b[1;2" => Partial,
    "\x1b[1;2A" => Shift(UpArrow),
    "\x1b[1;2B" => Shift(DownArrow),
    "\x1b[1;2C" => Shift(RightArrow),
    "\x1b[1;2D" => Shift(LeftArrow),
    "\x1b[1;2F" => Shift(End),
    "\x1b[1;2H" => Shift(Home),
    "\x1b[1;2P" => Shift(F1),
    "\x1b[1;2Q" => Shift(F2),
    "\x1b[1;2R" => Shift(F3),
    "\x1b[1;2S" => Shift(F4),
    "\x1b[1;3" => Partial,
    "\x1b[1;3A" => Alt(UpArrow),
    "\x1b[1;3B" => Alt(DownArrow),
    "\x1b[1;3C" => Alt(RightArrow),
    "\x1b[1;3D" => Alt(LeftArrow),
    "\x1b[1;3F" => Alt(End),
    "\x1b[1;3H" => Alt(Home),
    "\x1b[1;3P" => Alt(F1),
    "\x1b[1;3Q" => Alt(F2),
    "\x1b[1;3R" => Alt(F3),
    "\x1b[1;3S" => Alt(F4),
    "\x1b[1;4" => Partial,
    "\x1b[1;4A" => ShiftAlt(UpArrow),
    "\x1b[1;4B" => ShiftAlt(DownArrow),
    "\x1b[1;4C" => ShiftAlt(RightArrow),
    "\x1b[1;4D" => ShiftAlt(LeftArrow),
    "\x1b[1;4F" => ShiftAlt(End),
    "\x1b[1;4H" => ShiftAlt(Home),
    "\x1b[1;4P" => ShiftAlt(F1),
    "\x1b[1;4Q" => ShiftAlt(F2),
    "\x1b[1;4R" => ShiftAlt(F3),
    "\x1b[1;4S" => ShiftAlt(F4),
    "\x1b[1;5" => Partial,
    "\x1b[1;5A" => Ctrl(UpArrow),
    "\x1b[1;5B" => Ctrl(DownArrow),
    "\x1b[1;5C" => Ctrl(RightArrow),
    "\x1b[1;5D" => Ctrl(LeftArrow),
    "\x1b[1;5F" => Ctrl(End),
    "\x1b[1;5H" => Ctrl(Home),
    "\x1b[1;5P" => Ctrl(F1),
    "\x1b[1;5Q" => Ctrl(F2),
    "\x1b[1;5R" => Ctrl(F3),
    "\x1b[1;5S" => Ctrl(F4),
    "\x1b[1;6" => Partial,
    "\x1b[1;6A" => CtrlShift(UpArrow),
    "\x1b[1;6B" => CtrlShift(DownArrow),
    "\x1b[1;6C" => CtrlShift(RightArrow),
    "\x1b[1;6D" => CtrlShift(LeftArrow),
    "\x1b[1;6F" => CtrlShift(End),
    "\x1b[1;6H" => CtrlShift(Home),
    "\x1b[1;6P" => CtrlShift(F1),
    "\x1b[1;6Q" => CtrlShift(F2),
    "\x1b[1;6R" => CtrlShift(F3),
    "\x1b[1;6S" => CtrlShift(F4),
    "\x1b[1;7" => Partial,
    "\x1b[1;7A" => CtrlAlt(UpArrow),
    "\x1b[1;7B" => CtrlAlt(DownArrow),
    "\x1b[1;7C" => CtrlAlt(RightArrow),
    "\x1b[1;7D" => CtrlAlt(LeftArrow),
    "\x1b[1;7F" => CtrlAlt(End),
    "\x1b[1;7H" => CtrlAlt(Home),
    "\x1b[1;7P" => CtrlAlt(F1),
    "\x1b[1;7Q" => CtrlAlt(F2),
    "\x1b[1;7R" => CtrlAlt(F3),
    "\x1b[1;7S" => CtrlAlt(F4),
    "\x1b[1;8" => Partial,
    "\x1b[1;8A" => CtrlShiftAlt(UpArrow),
    "\x1b[1;8B" => CtrlShiftAlt(DownArrow),
    "\x1b[1;8C" => CtrlShiftAlt(RightArrow),
    "\x1b[1;8D" => CtrlShiftAlt(LeftArrow),
    "\x1b[1;8F" => CtrlShiftAlt(End),
    "\x1b[1;8H" => CtrlShiftAlt(Home),
    "\x1b[1;8P" => CtrlShiftAlt(F1),
    "\x1b[1;8Q" => CtrlShiftAlt(F2),
    "\x1b[1;8R" => CtrlShiftAlt(F3),
    "\x1b[1;8S" => CtrlShiftAlt(F4),
    "\x1b[1~" => Normal(Home),
    "\x1b[2" => Partial,
    "\x1b[20" => Partial,
    "\x1b[20;" => Partial,
    "\x1b[20;2" => Partial,
    "\x1b[20;2~" => Shift(F9),
    "\x1b[20;3" => Partial,
    "\x1b[20;3~" => Alt(F9),
    "\x1b[20;4" => Partial,
    "\x1b[20;4~" => ShiftAlt(F9),
    "\x1b[20;5" => Partial,
    "\x1b[20;5~" => Ctrl(F9),
    "\x1b[20;6" => Partial,
    "\x1b[20;6~" => CtrlShift(F9),
    "\x1b[20;7" => Partial,
    "\x1b[20;7~" => CtrlAlt(F9),
    "\x1b[20;8" => Partial,
    "\x1b[20;8~" => CtrlShiftAlt(F9),
    "\x1b[20~" => Normal(F9),
    "\x1b[21" => Partial,
    "\x1b[21;" => Partial,
    "\x1b[21;2" => Partial,
    "\x1b[21;2~" => Shift(F10),
    "\x1b[21;3" => Partial,
    "\x1b[21;3~" => Alt(F10),
    "\x1b[21;4" => Partial,
    "\x1b[21;4~" => ShiftAlt(F10),
    "\x1b[21;5" => Partial,
    "\x1b[21;5~" => Ctrl(F10),
    "\x1b[21;6" => Partial,
    "\x1b[21;6~" => CtrlShift(F10),
    "\x1b[21;7" => Partial,
    "\x1b[21;7~" => CtrlAlt(F10),
    "\x1b[21;8" => Partial,
    "\x1b[21;8~" => CtrlShiftAlt(F10),
    "\x1b[21~" => Normal(F10),
    "\x1b[23" => Partial,
    "\x1b[23;" => Partial,
    "\x1b[23;2" => Partial,
    "\x1b[23;2~" => Shift(F11),
    "\x1b[23;3" => Partial,
    "\x1b[23;3~" => Alt(F11),
    "\x1b[23;4" => Partial,
    "\x1b[23;4~" => ShiftAlt(F11),
    "\x1b[23;5" => Partial,
    "\x1b[23;5~" => Ctrl(F11),
    "\x1b[23;6" => Partial,
    "\x1b[23;6~" => CtrlShift(F11),
    "\x1b[23;7" => Partial,
    "\x1b[23;7~" => CtrlAlt(F11),
    "\x1b[23;8" => Partial,
    "\x1b[23;8~" => CtrlShiftAlt(F11),
    "\x1b[23~" => Normal(F11),
    "\x1b[24" => Partial,
    "\x1b[24;" => Partial,
    "\x1b[24;2" => Partial,
    "\x1b[24;2~" => Shift(F12),
    "\x1b[24;3" => Partial,
    "\x1b[24;3~" => Alt(F12),
    "\x1b[24;4" => Partial,
    "\x1b[24;4~" => ShiftAlt(F12),
    "\x1b[24;5" => Partial,
    "\x1b[24;5~" => Ctrl(F12),
    "\x1b[24;6" => Partial,
    "\x1b[24;6~" => CtrlShift(F12),
    "\x1b[24;7" => Partial,
    "\x1b[24;7~" => CtrlAlt(F12),
    "\x1b[24;8" => Partial,
    "\x1b[24;8~" => CtrlShiftAlt(F12),
    "\x1b[24~" => Normal(F12),
    "\x1b[2;" => Partial,
    "\x1b[2;2" => Partial,
    "\x1b[2;2~" => Shift(Insert),
    "\x1b[2;3" => Partial,
    "\x1b[2;3~" => Alt(Insert),
    "\x1b[2;4" => Partial,
    "\x1b[2;4~" => ShiftAlt(Insert),
    "\x1b[2;5" => Partial,
    "\x1b[2;5~" => Ctrl(Insert),
    "\x1b[2;6" => Partial,
    "\x1b[2;6~" => CtrlShift(Insert),
    "\x1b[2;7" => Partial,
    "\x1b[2;7~" => CtrlAlt(Insert),
    "\x1b[2;8" => Partial,
    "\x1b[2;8~" => CtrlShiftAlt(Insert),
    "\x1b[2~" => Normal(Insert),
    "\x1b[3" => Partial,
    "\x1b[3;" => Partial,
    "\x1b[3;2" => Partial,
    "\x1b[3;2~" => Shift(Delete),
    "\x1b[3;3" => Partial,
    "\x1b[3;3~" => Alt(Delete),
    "\x1b[3;4" => Partial,
    "\x1b[3;4~" => ShiftAlt(Delete),
    "\x1b[3;5" => Partial,
    "\x1b[3;5~" => Ctrl(Delete),
    "\x1b[3;6" => Partial,
    "\x1b[3;6~" => CtrlShift(Delete),
    "\x1b[3;7" => Partial,
    "\x1b[3;7~" => CtrlAlt(Delete),
    "\x1b[3;8" => Partial,
    "\x1b[3;8~" => CtrlShiftAlt(Delete),
    "\x1b[3~" => Normal(Delete),
    "\x1b[4" => Partial,
    "\x1b[4~" => Normal(End),
    "\x1b[5" => Partial,
    "\x1b[5;" => Partial,
    "\x1b[5;2" => Partial,
    "\x1b[5;2~" => Shift(PageUp),
    "\x1b[5;3" => Partial,
    "\x1b[5;3~" => Alt(PageUp),
    "\x1b[5;4" => Partial,
    "\x1b[5;4~" => ShiftAlt(PageUp),
    "\x1b[5;5" => Partial,
    "\x1b[5;5~" => Ctrl(PageUp),
    "\x1b[5;6" => Partial,
    "\x1b[5;6~" => CtrlShift(PageUp),
    "\x1b[5;7" => Partial,
    "\x1b[5;7~" => CtrlAlt(PageUp),
    "\x1b[5;8" => Partial,
    "\x1b[5;8~" => CtrlShiftAlt(PageUp),
    "\x1b[5~" => Normal(PageUp),
    "\x1b[6" => Partial,
    "\x1b[6;" => Partial,
    "\x1b[6;2" => Partial,
    "\x1b[6;2~" => Shift(PageDown),
    "\x1b[6;3" => Partial,
    "\x1b[6;3~" => Alt(PageDown),
    "\x1b[6;4" => Partial,
    "\x1b[6;4~" => ShiftAlt(PageDown),
    "\x1b[6;5" => Partial,
    "\x1b[6;5~" => Ctrl(PageDown),
    "\x1b[6;6" => Partial,
    "\x1b[6;6~" => CtrlShift(PageDown),
    "\x1b[6;7" => Partial,
    "\x1b[6;7~" => CtrlAlt(PageDown),
    "\x1b[6;8" => Partial,
    "\x1b[6;8~" => CtrlShiftAlt(PageDown),
    "\x1b[6~" => Normal(PageDown),
    "\x1b[A" => Normal(UpArrow),
    "\x1b[B" => Normal(DownArrow),
    "\x1b[C" => Normal(RightArrow),
    "\x1b[D" => Normal(LeftArrow),
    "\x1b[Z" => Shift(Tab),
    "\x1b\x7f" => Alt(Backspace),
    "\x7f" => Normal(Backspace),
    // MacOS Specific Encodings
    "\x1b\x1b" => Partial,
    "\x1b\x1b[" => Partial,
    "\x1b\x1b[3" => Partial,
    "\x1b\x1b[3;" => Partial,
    "\x1b\x1b[3;5" => Partial,
    "\x1b\x1b[3;5~" => CtrlAlt(Delete),
    "\x1b[F" => Normal(End),
    "\x1b[H" => Normal(Home),
};

/*
static U8_TO_PRINT: [&'static str; 256] = [
    "<CTRL-@>", "<CTRL-A>", "<CTRL-B>", "<CTRL-C>", "<CTRL-D>", "<CTRL-E>", "<CTRL-F>", "<CTRL-G>",
    "<CTRL-H>", "<CTRL-I>", "<CTRL-J>", "<CTRL-K>", "<CTRL-L>", "<CTRL-M>", "<CTRL-N>", "<CTRL-O>",
    "<CTRL-P>", "<CTRL-Q>", "<CTRL-R>", "<CTRL-S>", "<CTRL-T>", "<CTRL-U>", "<CTRL-V>", "<CTRL-W>",
    "<CTRL-X>", "<CTRL-Y>", "<CTRL-Z>", "<ESC>", "<CTRL-\\>", "<CTRL-]>", "<CTRL-^>", "<CTRL-_>",
    "\x20", "\x21", "\x22", "\x23", "\x24", "\x25", "\x26", "\x27",
    "\x28", "\x29", "\x2a", "\x2b", "\x2c", "\x2d", "\x2e", "\x2f",
    "\x30", "\x31", "\x32", "\x33", "\x34", "\x35", "\x36", "\x37",
    "\x38", "\x39", "\x3a", "\x3b", "\x3c", "\x3d", "\x3e", "\x3f",
    "\x40", "\x41", "\x42", "\x43", "\x44", "\x45", "\x46", "\x47",
    "\x48", "\x49", "\x4a", "\x4b", "\x4c", "\x4d", "\x4e", "\x4f",
    "\x50", "\x51", "\x52", "\x53", "\x54", "\x55", "\x56", "\x57",
    "\x58", "\x59", "\x5a", "\x5b", "\x5c", "\x5d", "\x5e", "\x5f",
    "\x60", "\x61", "\x62", "\x63", "\x64", "\x65", "\x66", "\x67",
    "\x68", "\x69", "\x6a", "\x6b", "\x6c", "\x6d", "\x6e", "\x6f",
    "\x70", "\x71", "\x72", "\x73", "\x74", "\x75", "\x76", "\x77",
    "\x78", "\x79", "\x7a", "\x7b", "\x7c", "\x7d", "\x7e", "<DEL>",
    "<80>", "<81>", "<82>", "<83>", "<84>", "<85>", "<86>", "<87>",
    "<88>", "<89>", "<8A>", "<8B>", "<8C>", "<8D>", "<8E>", "<8F>",
    "<90>", "<91>", "<92>", "<93>", "<94>", "<95>", "<96>", "<97>",
    "<98>", "<99>", "<9A>", "<9B>", "<9C>", "<9D>", "<9E>", "<9F>",
    "<A0>", "<A1>", "<A2>", "<A3>", "<A4>", "<A5>", "<A6>", "<A7>",
    "<A8>", "<A9>", "<AA>", "<AB>", "<AC>", "<AD>", "<AE>", "<AF>",
    "<B0>", "<B1>", "<B2>", "<B3>", "<B4>", "<B5>", "<B6>", "<B7>",
    "<B8>", "<B9>", "<BA>", "<BB>", "<BC>", "<BD>", "<BE>", "<BF>",
    "<C0>", "<C1>", "<C2>", "<C3>", "<C4>", "<C5>", "<C6>", "<C7>",
    "<C8>", "<C9>", "<CA>", "<CB>", "<CC>", "<CD>", "<CE>", "<CF>",
    "<D0>", "<D1>", "<D2>", "<D3>", "<D4>", "<D5>", "<D6>", "<D7>",
    "<D8>", "<D9>", "<DA>", "<DB>", "<DC>", "<DD>", "<DE>", "<DF>",
    "<E0>", "<E1>", "<E2>", "<E3>", "<E4>", "<E5>", "<E6>", "<E7>",
    "<E8>", "<E9>", "<EA>", "<EB>", "<EC>", "<ED>", "<EE>", "<EF>",
    "<F0>", "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>",
    "<F8>", "<F9>", "<FA>", "<FB>", "<FC>", "<FD>", "<FE>", "<FF>",
];
*/

pub fn encode_extended_string(s: &str) -> ExtendedChars {
    // First, check to see if there are any control characters in the string
    let mut temp_str = String::new();
    let mut ext_str: Vec<ExtendedChars> = Vec::new();
    let mut escape_last = false;
    for c in s.chars() {
        if c.is_ascii_control() {
            if '\x1b' == c {
                if escape_last {
                    ext_str.push(Alt(Escape));
                    escape_last = false;
                }
                else {
                    if !temp_str.is_empty() {
                        ext_str.push(CharString(temp_str.clone()));
                        temp_str.clear();
                    }
                    escape_last = true;
                }
            }
            else  {
                let mut b = [0; 1];
                c.encode_utf8(&mut b);
                let special = if 0x20 > b[0] {
                    b[0] += b'@';
                    if escape_last {
                        escape_last = false;
                        CtrlAltChar(char::from(b[0]))
                    }
                    else {
                        CtrlChar(char::from(b[0]))
                    }
                }
                else {
                    if escape_last {
                        ext_str.push(Normal(Escape));
                        escape_last = false;
                    }
                    Byte(b[0])
                };
                if !temp_str.is_empty() {
                    ext_str.push(CharString(temp_str.clone()));
                    temp_str.clear();
                }
                ext_str.push(special);
            };
        }
        else {
            if escape_last {
                ext_str.push(AltChar(c));
                escape_last = false;
            }
            else {
                temp_str.push(c);
            }
        }
    }
    if escape_last {
        ext_str.push(Normal(Escape));
    }
    if ext_str.is_empty() {
        CharString(temp_str)
    }
    else {
        if !temp_str.is_empty() {
            ext_str.push(CharString(temp_str));
        }
        if 1 == ext_str.len() {
            ext_str.pop().unwrap()
        }
        else {
            ExtendedString(ext_str)
        }
    }
}

impl FromStr for ExtendedChars {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match CHAR_CODES.get(s) {
            Some(t) => Ok(t.clone()),
            None => Ok(encode_extended_string(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMapError {
    pub(super) kind: MapErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapErrorKind {
    NotFound,
}

impl ParseMapError {
    pub fn __description(&self) -> &str {
        match self.kind {
            MapErrorKind::NotFound => "input doesn't match",
        }
    }
}

impl Display for ParseMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl Error for ParseMapError {}
