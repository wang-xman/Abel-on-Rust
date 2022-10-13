use std::fmt;

use crate::error::ErrorKind;
use crate::typefy::NamedType;
use crate::symbol;
use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsonTokenType {
    Key,
    IterKey,
    Terminal,
    DictOpening,
    DictClosing,
    ListOpening,
    ListClosing,
}

impl NamedType for JsonTokenType {
    fn type_name(&self) -> &'static str {
        match *self {
            Self::Key => "Key",
            Self::IterKey => "IterKey",
            Self::Terminal => "Terminal",
            Self::DictOpening => "DictOpening",
            Self::DictClosing => "DictClosing",
            Self::ListOpening => "ListOpening",
            Self::ListClosing => "ListClosing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsonContainerType {
    None,
    Dict,
    List,
}

impl NamedType for JsonContainerType {
    fn type_name(&self) -> &'static str {
        match *self {
            Self::None => "None",
            Self::Dict => "Dict",
            Self::List => "List",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralScheme {
    None,
    Delimited,
    Liberal
}

impl NamedType for LiteralScheme {
    fn type_name(&self) -> &'static str {
        match *self {
            Self::None => "None",
            Self::Delimited => "Delimited",
            Self::Liberal => "Liberal",
        }
    }
}

/// Terminal types for both standard JSON and JSON+ formats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JsonTerminalType {
    NotSet,
    Null,
    Bool,
    Text,
    Integer,
    Double,
    Complex,
    Binary,
    Bitstring,
}

impl NamedType for JsonTerminalType {
    fn type_name(&self) -> &'static str {
        match *self {
            Self::NotSet => "",
            Self::Null => "Null",
            Self::Bool => "Bool",
            Self::Text => "Text",
            Self::Integer => "Integer",
            Self::Double => "Double",
            Self::Complex => "Complex",
            Self::Binary => "Binary",
            Self::Bitstring => "Bitstring",
        }
    }
}

/// TODO 1
#[derive(Clone)]
pub struct JsonToken {
    pub literal: String,
    pub token_type: JsonTokenType,
    pub parent_key: String,
    pub level: i32,
    pub line: i32,
    pub container_type: JsonContainerType,
    pub iter_index: i32,
    pub terminal_type: JsonTerminalType,
    pub literal_scheme: LiteralScheme,
    pub referenced_type: String,
}

impl JsonToken {
    pub fn get_type(&self) -> JsonTokenType {
        self.token_type
    }
}

impl Token for JsonToken {
    fn type_name(&self) -> &'static str {
        self.token_type.type_name()
    }

    fn token_to_string(&self) -> String {
        let stem = format!(
            "Line:{}, Literal:{}, Type:{}, PK:{}, Level:{}, ContainerType:{}",
            self.line.to_string(), self.literal, self.type_name(), self.parent_key,
            self.level.to_string(),self.container_type.type_name());
        match self.type_name() {
            "IterKey" => format!(
                "({}, IterIndex:{}, ReferencedType:{})",
                stem,
                self.iter_index.to_string(),
                self.referenced_type),
            "Terminal" => format!(
                "({}, TerminalType:{}, LiteralScheme:{})",
                stem,
                self.terminal_type.type_name(),
                self.literal_scheme.type_name()),
            "Key" => format!(
                "({}, ReferencedType:{}, LiteralScheme:{})",
                stem,
                self.referenced_type,
                self.literal_scheme.type_name()),
            _ => format!("({})", stem)
        }
    }
}

impl fmt::Display for JsonToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token_to_string())
    }
}

/// Tokenize an iter key token
pub fn tokenize_iter_key(
    literal: &str,
    parent_key: &str,
    level: i32,
    line: i32,
    container_type: JsonContainerType,
    iter_index: i32
) -> JsonToken {
    JsonToken {
        literal: literal.to_owned(),
        token_type: JsonTokenType::IterKey,
        parent_key: parent_key.to_owned(),
        level: level,
        line: line,
        container_type: container_type,
        iter_index: iter_index,
        terminal_type: JsonTerminalType::NotSet, // default
        literal_scheme: LiteralScheme::None, // default
        referenced_type: "".to_owned() // default
    }
}

/// Tokenize a terminal token
pub fn tokenize_terminal(
    literal: &str,
    parent_key: &str,
    level: i32,
    line: i32,
    container_type: JsonContainerType,
    terminal_type: JsonTerminalType,
    scheme: LiteralScheme
) -> JsonToken {
    JsonToken {
        literal: literal.to_owned(),
        token_type: JsonTokenType::Terminal,
        parent_key: parent_key.to_owned(),
        level: level,
        line: line,
        container_type: container_type,
        iter_index: -99, // default
        terminal_type: terminal_type,
        literal_scheme: scheme,
        referenced_type: "".to_owned() // default
    }
}

pub fn tokenize_key(
    literal: &str,
    parent_key: &str,
    level: i32,
    line: i32,
    container_type: JsonContainerType,
    scheme: LiteralScheme
) -> JsonToken {
    JsonToken {
        literal: literal.to_owned(),
        token_type: JsonTokenType::Key,
        parent_key: parent_key.to_owned(),
        level: level,
        line: line,
        container_type: container_type,
        iter_index: -99,
        terminal_type: JsonTerminalType::NotSet, // default
        literal_scheme: scheme,
        referenced_type: "".to_owned() // default
    }
}

pub fn tokenize(
    literal: &str,
    token_type: JsonTokenType,
    parent_key: &str,
    level: i32,
    line: i32,
    container_type: JsonContainerType
) -> JsonToken {
    JsonToken {
        literal: literal.to_owned(),
        token_type: token_type,
        parent_key: parent_key.to_owned(),
        level: level,
        line: line,
        container_type: container_type,
        iter_index: -99,
        terminal_type: JsonTerminalType::NotSet, // default
        literal_scheme: LiteralScheme::None, // default
        referenced_type: "".to_owned() // default
    }
}

pub fn get_token_type_by_symbol(symbol_char: char)
-> Result<JsonTokenType, ErrorKind>
{
    match symbol_char {
        symbol::L_BRACE => Ok(JsonTokenType::DictOpening),
        symbol::R_BRACE => Ok(JsonTokenType::DictClosing),
        symbol::L_BRACKET => Ok(JsonTokenType::ListOpening),
        symbol::R_BRACKET => Ok(JsonTokenType::ListClosing),
        _ => Err(ErrorKind::UnrecognizedSymbol),
    }
}

pub fn get_container_type_by_symbol(opening_symbol: char)
-> Result<JsonContainerType, ErrorKind>
{
    match opening_symbol {
        symbol::L_BRACE => Ok(JsonContainerType::Dict),
        symbol::L_BRACKET => Ok(JsonContainerType::List),
        _ => Err(ErrorKind::UnrecognizedSymbol),
    }
}

#[cfg(test)]
#[path = "./unittest/json_token/tests.rs"]
mod tests;