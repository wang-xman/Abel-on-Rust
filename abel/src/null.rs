use std::fmt;
use std::any::Any;

use crate::error::{Error, InternalError, ErrorKind};
use crate::object::Object;
use crate::marker::ScalarValued;

#[derive(Debug, PartialEq)]
pub struct Null {
    internal: i32,
}

impl Null {
    pub fn new() -> Self {
        Null {
            internal: 0
        }
    }

    pub fn from_str(src_str: &str) -> Result<Self, InternalError> {
        match src_str {
            "null" => Ok(Self::new()),
            _ => Err(InternalError::new(
                "Failed to identify and convert a Null type from string.",
                ErrorKind::FailedToIdentify
            ))
        }
    }

    pub fn as_str(&self) -> &'static str {
        "null"
    }
}

impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Null"
    }
}

impl ScalarValued<i32> for Null {
    fn value(&self) -> i32 {
        self.internal
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "null")
    }
}

// Unittest
#[cfg(test)]
#[path = "./unittest/null/tests.rs"]
mod tests;