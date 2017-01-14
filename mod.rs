#![allow(dead_code)]

use ::libc;

/// The limit of draws by sprite.
pub const SPEC_MAX_DRAW: usize = 16;

pub const SPEC_MAX_X: usize = 10;
pub const SPEC_MAX_Y: usize = 5;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;
pub const SPEC_MAX_PRE_XY: usize = SPEC_MAX_XY - 1;

pub const SPEC_CHARACTER_MAX: usize = SPEC_MAX_Y * 16;

pub const BLACK: [libc::c_uchar; 3] = [0, 0, 0];
pub const RED: [libc::c_uchar; 3] = [255, 0, 0];
pub const YELLOW: [libc::c_uchar; 3] = [255, 255, 0];
pub const GREEN: [libc::c_uchar; 3] = [0, 255, 0];
pub const CYAN: [libc::c_uchar; 3] = [0, 255, 255];
pub const BLUE: [libc::c_uchar; 3] = [0, 0, 255];
pub const MAGENTA: [libc::c_uchar; 3] = [255, 0, 255];
pub const WHITE: [libc::c_uchar; 3] = [255, 255, 255];

pub const DEFAULT_FOREGROUND: [libc::c_uchar; 3] = BLACK;
pub const DEFAULT_BACKGROUND: [libc::c_uchar; 3] = WHITE;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Attribute {
    None = 0x00,
    Dim = 0x02,
    Bold = 0x01,
    Italic = 0x04,
    Underline = 0x08,
    Blink = 0x10,
    Reverse = 0x20,
    Hidden = 0x40,
}

#[repr(u8)]
pub enum Cardinal {
    UpperLeft = 0,
    UpperMiddle = 1,
    UpperRight = 2,
    MiddleLeft = 3,
    MiddleCentral = 4,
    MiddleRight = 5,
    LowerLeft = 6,
    LowerMiddle = 7,
    LowerRight = 8,
}

#[repr(C)]
pub struct Position {
    pub cardinal: Cardinal,
    pub cartesian: [libc::c_ushort; 2],
}

#[repr(u8)]
pub enum Sheet {
    None = b'_',
    Bust = b'b',
}

#[repr(C)]
pub struct Tuple {
    pub part: Part,
    pub emotion: Emotion,
}

#[repr(u8)]
pub enum Part {
    None = b'_',
    ArmLeft = b'a',
    ArmRight = b'A',
    Boobs = b'b',
    Clavicle = b'c',
    EarLeft = b'e',
    EarRight = b'E',
    EyeLeft = b'y',
    EyeRight = b'Y',
    HairTop = b'o',
    HairLeft = b'r',
    HairRight = b'R',
    HandLeft = b'd',
    HandRight = b'D',
    Mouth = b'm',
    Tail = b't',
    Bell = b'l',
    ExclamationMark = b'x',
    ExclamationMarks = b'X',
    Heart = b'h',
    Hearts = b'H',
    Lantern = b'n',
    QuestionMark = b'q',
    QuestionMarks = b'Q',
    WoolBall = b'w',
}

#[repr(u8)]
pub enum Emotion {
    None = b'_',
    Angry = b'a',
    Happy = b'h',
    Love = b'l',
    Malicious = b'm',
    Misunderstanding = b'i',
    Normal = b'n',  
    Playing = b'p',
    Shocked = b'o',
    Sleepy = b's',
    Speechless = b'e',
    Surprised = b'u',
}

#[repr(u8)]
pub enum Relative {
    Top = 0,
    Bottom = 1,
    Right = 2,
    Left = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Character {
    /// Attribute.
    attribute: libc::c_uchar,
    /// Text color.
    foreground: [libc::c_uchar; 3],
    /// Background color.
    background: [libc::c_uchar; 3],
    /// Glyph.
    glyph: libc::c_uint,
}

#[repr(C)]
pub struct Personnage {
    pub sheet: Sheet,
    pub emotion: [[Tuple; SPEC_MAX_XY]; SPEC_MAX_DRAW],
    pub position: Position,
}

#[repr(C)]
pub struct Say {
    pub cardinal: Relative,
    pub message: [Character; SPEC_CHARACTER_MAX],
}

#[repr(C)]
pub struct LibraryState {
    pub neko: Personnage,
    pub infobulle: Say,
    pub unmount: libc::u_char,
    pub lock: libc::u_char,
}
