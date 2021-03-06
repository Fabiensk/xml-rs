//! Contains several types used throughout the library.

use std::fmt;
use std::error;

/// Represents a position inside some textual document.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TextPosition {
    /// Row, counting from 0
    pub row: u64,
    /// Column, counting from 0
    pub column: u64,
}

impl TextPosition {
    /// Creates a new position initialized to the beginning of the document
    #[inline]
    pub fn new() -> TextPosition {
        TextPosition { row: 0, column: 0 }
    }

    /// Advances the position in a line
    #[inline]
    pub fn advance(&mut self, count: u8) {
        self.column += count as u64;
    }

    /// Advances the position in a line to the next tab position
    #[inline]
    pub fn advance_to_tab(&mut self, width: u8) {
        let width = width as u64;
        self.column += width - self.column % width
    }

    /// Advances the position to the beginning of the next line
    #[inline]
    pub fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;
    }
}

impl fmt::Debug for TextPosition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

impl fmt::Display for TextPosition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

/// Get the position in the document corresponding to the object
///
/// This trait is implemented by parsers, lexers and errors.
pub trait Position {
    /// Returns the current position or a position corresponding to the object.
    fn position(&self) -> TextPosition;
}

impl Position for TextPosition {
    #[inline]
    fn position(&self) -> TextPosition {
        *self
    }
}

/// XML parsing error.
///
/// Consists of a position and a message.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error {
    pos: TextPosition,
    msg: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.pos, self.msg)
    }
}

impl Position for Error {
    #[inline]
    fn position(&self) -> TextPosition { self.pos }
}

impl Error {
    /// Creates a new error using position information from the provided
    /// `Position` object and a message.
    #[inline]
    pub fn new<O: Position>(o: &O, msg: String) -> Error {
        Error { pos: o.position(), msg: msg }
    }

    /// Returns a reference to a message which is contained inside this error.
    #[inline]
    pub fn msg<'a>(&'a self) -> &'a str { &self.msg }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str { &*self.msg }
}

/// XML version enumeration.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum XmlVersion {
    /// XML version 1.0.
    Version10,

    /// XML version 1.1.
    Version11
}

impl fmt::Display for XmlVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XmlVersion::Version10 => write!(f, "1.0"),
            XmlVersion::Version11 => write!(f, "1.1")
        }
    }
}


/// Checks whether the given character is a white space character (`S`)
/// as is defined by XML 1.1 specification, [section 2.3][1].
///
/// [1]: http://www.w3.org/TR/2006/REC-xml11-20060816/#sec-common-syn
pub fn is_whitespace_char(c: char) -> bool {
    match c {
        '\x20' | '\x09' | '\x0d' | '\x0a' => true,
        _ => false
    }
}

/// Checks whether the given character is a name start character (`NameStartChar`)
/// as is defined by XML 1.1 specification, [section 2.3][1].
///
/// [1]: http://www.w3.org/TR/2006/REC-xml11-20060816/#sec-common-syn
pub fn is_name_start_char(c: char) -> bool {
    match c {
        ':' | 'A'...'Z' | '_' | 'a'...'z' |
        '\u{C0}'...'\u{D6}' | '\u{D8}'...'\u{F6}' | '\u{F8}'...'\u{2FF}' |
        '\u{370}'...'\u{37D}' | '\u{37F}'...'\u{1FFF}' |
        '\u{200C}'...'\u{200D}' | '\u{2070}'...'\u{218F}' |
        '\u{2C00}'...'\u{2FEF}' | '\u{3001}'...'\u{D7FF}' |
        '\u{F900}'...'\u{FDCF}' | '\u{FDF0}'...'\u{FFFD}' |
        '\u{10000}'...'\u{EFFFF}' => true,
        _ => false
    }
}

/// Checks whether the given character is a name character (`NameChar`)
/// as is defined by XML 1.1 specification, [section 2.3][1].
///
/// [1]: http://www.w3.org/TR/2006/REC-xml11-20060816/#sec-common-syn
pub fn is_name_char(c: char) -> bool {
    match c {
        _ if is_name_start_char(c) => true,
        '-' | '.' | '0'...'9' | '\u{B7}' |
        '\u{300}'...'\u{3F6}' | '\u{203F}'...'\u{2040}' => true,
        _ => false
    }
}

