#![allow(dead_code)]

use ::libc;

/// The limit of draws by sprite.
pub const SPEC_MAX_DRAW: usize = 16;

pub const SPEC_MAX_X: usize = 10;
pub const SPEC_MAX_Y: usize = 7;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;

pub const SPEC_CHARACTER_MAX: usize = 1024;

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

#[repr(u32)]
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

#[repr(u32)]
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

#[repr(u32)]
pub enum Sheet {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'A'.
    BustAngry = 0x41,
    /// Symbol 'H'.
    BustHappy = 0x48,
    /// Symbol 'L'.
    BustLove = 0x4c,
    /// Symbol 'M'.
    BustMalicious = 0x4d,
    /// Symbol 'm'.
    BustMisunderstanding = 0x6d,
    /// Symbol 'n'.
    BustNormal = 0x6e,
    /// Symbol 'P'.
    BustPlaying = 0x50,
    /// Symbol 'S'.
    BustShocked = 0x53,
    /// Symbol 's'.
    BustSleepy = 0x73,
    /// Symbol 'p'.
    BustSpeechless = 0x70, 
    /// Symbol 'U'.
    BustSurprised = 0x55,
    /// Symbol 'a'.
    LyingAngry = 0x61,
    /// Symbol 'h'.
    LyingHappy = 0x68,
    /// Symbol 'l'.
    LyingLove = 0x6c,
    /// Symbol 'I'.
    LyingMalicious = 0x49,
    /// Symbol 'i'.
    LyingMisunderstanding = 0x69,
    /// Symbol 'N'.
    LyingNormal = 0x4e,
    /// Symbol 'R'.
    LyingPlaying = 0x52,
    /// Symbol 'D'.
    LyingShocked = 0x44,
    /// Symbol 'E'.
    LyingSleepy = 0x45,
    /// Symbol 'C'.
    LyingSpeechless = 0x43,
    /// Symbol 'u'.
    LyingSurprised = 0x75,
    /// Symbol 'G'.
    SeizaAngry = 0x47,
    /// Symbol 'Y'.
    SeizaHappy = 0x59,
    /// Symbol 'V'.
    SeizaLove = 0x56,
    /// Symbol 'c'.
    SeizaMalicious = 0x63,
    /// Symbol 'e'.
    SeizaMisunderstanding = 0x65,
    /// Symbol 'o'.
    SeizaNormal = 0x6f,
    /// Symbol 'g'.
    SeizaPlaying = 0x67,
    /// Symbol 'K'.
    SeizaShocked = 0x4b,
    /// Symbol 'y'.
    SeizaSleepy = 0x79,
    /// Symbol 'Z'.
    SeizaSpeechless = 0x5a,
    /// Symbol 'd'.
    SeizaSurprised = 0x64,
}

#[repr(C)]
pub struct Tuple {
    pub part: Part,
    pub emotion: Emotion,
}

#[repr(u32)]
pub enum Part {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'a'.
    ArmLeft = 0x61,
    /// Symbol 'A'.
    ArmRight = 0x41,
    /// Symbol 'b'.
    Boobs = 0x62,
    /// Symbol 'c'.
    Clavicle = 0x63,
    /// Symbol 'e'.
    EarLeft = 0x65,
    /// Symbol 'E'.
    EarRight = 0x45,
    /// Symbol 'y'.
    EyeLeft = 0x79,
    /// Symbol 'Y'.
    EyeRight = 0x59,
    /// Symbol 'o'.
    HairTop = 0x6f,
    /// Symbol 'r'.
    HairLeft = 0x72,
    /// Symbol 'R'.
    HairRight = 0x52,
    /// Symbol 'd',
    HandLeft = 0x64,
    /// Symbol 'D',
    HandRight = 0x44,
    /// Symbol 'm',
    Mouth = 0x6d,
    /// Symbol 't'.
    Tail = 0x74,
    /// Symbol 'l'.
    Bell = 0x6c,
    /// Symbol 'x'.
    ExclamationMark = 0x78,
    /// Symbol 'X'.
    ExclamationMarks = 0x58,
    /// Symbol 'h'.
    Heart = 0x68,
    /// Symbol 'H'.
    Hearts = 0x48,
    /// Symbol 'n'.
    Lantern = 0x6e,
    /// Symbol 'q'.
    QuestionMark = 0x71,
    /// Symbol 'Q'.
    QuestionMarks = 0x51,
    /// Symbol 'w'.
    WoolBall = 0x77,
}

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Emotion {
    /// Symbol '_'.
    None = 0x5f,
    /// Symbol 'a'.
    Angry = 0x61,
    /// Symbol 'h'.
    Happy = 0x68,
    /// Symbol 'l'.
    Love = 0x6c,
    /// Symbol 'm'.
    Malicious = 0x6d,
    /// Symbol 'i'.
    Misunderstanding = 0x69,
    /// Symbol 'o'.
    Shocked = 0x6f,
    /// Symbol 's'.
    Sleepy = 0x73,
    /// Symbol 'e'.
    Speechless = 0x65,
}

#[repr(u32)]
pub enum Relative {
    Top = 0,
    Bottom = 1,
    Right = 2,
    Left = 3,
}

#[repr(C)]
pub struct Character {
    /// Attribute.
    pub attribute: libc::c_uchar,
    /// Text color.
    pub foreground: [libc::c_uchar; 3],
    /// Background color.
    pub background: [libc::c_uchar; 3],
    /// Glyph.
    pub glyph: libc::c_uint,
}

#[repr(u32)]
pub enum Mouse {
    /// The left mouse button is pressed.
    Left = 0,
    /// The right mouse button is pressed.
    Right = 1,
    /// The mouse wheel button is pressed.
    Wheel = 2,
    /// The mouse wheel is going up.
    WheelUp = 64,
    /// The mouse wheel is going down.
    WheelDown = 65,
    /// The left mouse button is held while moving pointer.
    LeftDrag = 32,
    /// The wheel mouse button is held while moving pointer.
    WheelDrag = 33,
    /// The right mouse button is held while moving pointer.
    RightDrag = 34,
    /// The left mouse button is pressed while helding Shift.
    ShiftLeft = 4,
    /// The wheel mouse button is pressed while helding Shift.
    ShiftWheel = 5,
    /// The right mouse button is pressed while helding Shift.
    ShiftRight = 6,
    /// The left mouse button and Shift are held while moving pointer.
    ShiftLeftDrag = 36,
    /// The wheel mouse button and Shift are held while moving pointer.
    ShiftWheelDrag = 37,
    /// The right mouse button and Shift are held while moving pointer.
    ShiftRightDrag = 38,
    /// The left mouse button is pressed while helding Ctrl
    CtrlLeft = 16,
    /// The wheel mouse button is pressed while helding Ctrl
    CtrlWheel = 17,
    /// The right mouse button is pressed while helding Ctrl
    CtrlRight = 18,
    /// The mouse wheel is going up while helding Ctrl
    CtrlWheelUp = 80,
    /// The mouse wheel is going down while helding Ctrl
    CtrlWheelDown = 81,
    /// The left mouse button and Ctrl are held while moving pointer
    CtrlLeftDrag = 48,
    /// The wheel mouse button and Ctrl are held while moving pointer
    CtrlWheelDrag = 49,
    /// The right mouse button and Ctrl are held while moving pointer
    CtrlRightDrag = 50,
    /// The left mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlLeft = 20,
    /// The wheel mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlWheel = 21,
    /// The right mouse button is pressed while Ctrl and Shift are held
    ShiftCtrlRight = 22,
    /// The left mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlLeftDrag = 52,
    /// The wheel mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlWheelDrag = 53,
    /// The right mouse button, Ctrl and Shift are held while moving pointer
    ShiftCtrlRightDrag = 54,
    /// The left mouse button is pressed while helding Command
    CmdLeft = 8,
    /// The wheel mouse button is pressed while helding Command
    CmdWheel = 9,
    /// The right mouse button is pressed while helding Command
    CmdRight = 10,
    /// The mouse wheel is going up while helding Command
    CmdWheelUp = 72,
    /// The mouse wheel is going down while helding Command
    CmdWheelDown = 73,
    /// The left mouse button and Command are held while moving pointer
    CmdLeftDrag = 40,
    /// The wheel mouse button and Command are held while moving pointer
    CmdWheelDrag = 41,
    /// The right mouse button and Command are held while moving pointer
    CmdRightDrag = 42,
    /// The left mouse button is pressed while helding Command and Shift
    CmdShiftLeft = 12,
    /// The wheel mouse button is pressed while helding Command and Shift
    CmdShiftWheel = 13,
    /// The right mouse button is pressed while helding Command and Shift
    CmdShiftRight = 14,
    /// The left mouse button, Shift and Command are held while moving pointer
    CmdShiftLeftDrag = 44,
    /// The wheel mouse button, Shift and Command are held while moving pointer
    CmdShiftWheelDrag = 45,
    /// The right mouse button, Shift and Command are held while moving pointer
    CmdShiftRightDrag = 46,
    /// The left mouse button is pressed while helding Command and Ctrl
    CmdCtrlLeft = 24,
    /// The wheel mouse button is pressed while helding Command and Ctrl
    CmdCtrlWheel = 25,
    /// The right mouse button is pressed while helding Command and Ctrl
    CmdCtrlRight = 26,
    /// The mouse wheel is going up while helding Command and Ctrl
    CmdCtrlWheelUp = 88,
    /// The mouse wheel is going down while helding Command and Ctrl
    CmdCtrlWheelDown = 89,
    /// The left mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlLeftDrag = 56,
    /// The wheel mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlWheelDrag = 57,
    /// The right mouse button, Ctrl and Command are held while moving pointer
    CmdCtrlRightDrag = 58,
    /// The left mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlLeft = 28,
    /// The wheel mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlWheel = 29,
    /// The right mouse button is pressed while helding Command, Shift and Ctrl
    CmdShiftCtrlRight = 30,
    /// The left mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlLeftDrag = 60,
    /// The wheel mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlWheelDrag = 61,
    /// The right mouse button, Ctrl, Shift and Command are held while moving pointer
    CmdShiftCtrlRightDrag = 62,
}

#[repr(C)]
pub struct Winszed {
    /// Rows, in characters.
    pub ws_row: libc::c_ushort,
    /// Columns, in characters.
    pub ws_col: libc::c_ushort,
    /// Horizontal size, pixels.
    pub ws_xpixel: libc::c_ushort,
    /// Vertical size, pixels.
    pub ws_ypixel: libc::c_ushort, 
}

#[repr(C)]
pub struct Persona {
    pub sheet: Sheet,
    pub emotion: [[Tuple; SPEC_MAX_XY]; SPEC_MAX_DRAW],
    pub position: Position,
}

#[repr(C)]
pub struct Tooltip {
    pub cardinal: Relative,
    pub message: [Character; SPEC_CHARACTER_MAX],
}

#[repr(C)]
pub struct LibraryState {
    pub persona: Persona,
    pub tooltip: Tooltip,
    pub unmount: libc::c_uchar,
    pub lock: libc::c_uchar,
}
