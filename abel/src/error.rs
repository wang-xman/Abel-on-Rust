use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IncompatibleType,
    IndexOutOfRange,
    MismatchedType,
    KeyNotFound,
    DuplicateKey,
    UnrecognizedSymbol,
    TokenizationError,
    SyntaxError,
    UnpairedBrackets,
    VariableNotFound,
    InvalidInput,
    FailedToIdentify,
    FileNotFound,
}
 
impl ErrorKind {
    pub fn get_enum_type(&self) -> &'static str {
        "ErrorKind"
    }

    pub fn get_header(&self) -> &'static str {
        match *self {
            ErrorKind::IncompatibleType => "INCOMPATIBLE_TYPE",
            ErrorKind::IndexOutOfRange => "INDEX_OUT_OF_RANGE",
            ErrorKind::MismatchedType => "MISMATCHED_TYPE",
            ErrorKind::KeyNotFound => "KEY_NOT_FOUND",
            ErrorKind::DuplicateKey => "DUPLICATE_KEY",
            ErrorKind::UnrecognizedSymbol => "UNRECOGNIZED_SYMBOL",
            ErrorKind::TokenizationError => "TOKENIZATION_ERROR",
            ErrorKind::SyntaxError => "SYNTAX_ERROR",
            ErrorKind::UnpairedBrackets => "UNPAIRED_BRACKETS",
            ErrorKind::VariableNotFound => "VARIABLE_NOT_FOUND",
            ErrorKind::InvalidInput => "INVALID_INPUT",
            ErrorKind::FailedToIdentify => "FAILED_TO_IDENTIFY",
            ErrorKind::FileNotFound => "FILE_NOT_FOUND",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_header())
    }
}

/// All specialised errors must implement trait Error.
pub trait Error {
    fn new(msg: &str, error_kind: ErrorKind) -> Self;

    fn set_msg(&mut self, msg: &str);

    fn get_msg(&self) -> &str;

    fn full_message(&self) -> String;

    fn show(&self) {
        println!("{}", self.full_message());
    }
}

/// Internal error
#[derive(Debug, PartialEq)]
pub struct InternalError {
    message: String,
    error_kind: ErrorKind,
}

impl Error for InternalError {
    fn new(msg: &str, error_kind: ErrorKind) -> Self {
        InternalError {
            message: msg.to_string(),
            error_kind
        }
    }

    fn set_msg(&mut self, msg: &str) {
        self.message = msg.to_string();
    }

    fn get_msg(&self) -> &str {
        &self.message
    }

    fn full_message(&self) -> String {
        format!("{}({})", self.get_header(), self.get_msg())
    }
}

impl InternalError {
    pub fn set_type(&mut self, error_kind: ErrorKind) {
        self.error_kind = error_kind;
    }

    pub fn get_header(&self) -> &'static str {
        self.error_kind.get_header()
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

/// Parser error
#[derive(Debug, PartialEq)]
pub struct ParserError {
    message: String,
    line_number: i32,
    error_kind: ErrorKind,
}

impl Error for ParserError {
    fn new(msg: &str, error_kind: ErrorKind) -> Self {
        ParserError {
            message: msg.to_string(),
            line_number: -99, // default to -99
            error_kind: error_kind,
        }
    }
    
    fn set_msg(&mut self, msg: &str) {
        self.message = msg.to_string();
    }

    fn get_msg(&self) -> &str {
        &self.message
    }

    fn full_message(&self) -> String {
        if self.get_line() < 0 {
            format!("{}({})", self.get_header(), self.get_msg())
        } else {
            format!("{}(Line {}, {}).", self.get_header(),
                    self.get_line(), self.get_msg() )
        }
    }
}

impl ParserError {
    pub fn set_line(&mut self, line: i32) {
        self.line_number = line;
    }

    pub fn set_type(&mut self, error_kind: ErrorKind) {
        self.error_kind = error_kind;
    }

    pub fn get_line(&self) -> i32 {
        self.line_number
    }

    pub fn get_header(&self) -> &'static str {
        self.error_kind.get_header()
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

// Unittest
#[cfg(test)]
#[path = "./unittest/error/tests.rs"]
mod tests;