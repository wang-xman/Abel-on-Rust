use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::error::{Error, InternalError, ErrorKind};
use crate::object::Object;
use crate::typefy::IntoType;
use crate::marker::ScalarValued;

#[derive(Debug, PartialEq)]
pub struct Bool {
    internal: bool,
}

impl Bool {
    pub fn new(internal: bool) -> Self {
        Bool {
            internal
        }
    }

    pub fn from(internal: bool) -> Self {
        Bool {
            internal
        }
    }

    pub fn from_str(src_str: &str) -> Result<Self, InternalError> {
        match src_str {
            "true" => Ok(Self::new(true)),
            "false" => Ok(Self::new(false)),
            _ => Err(InternalError::new(
                "Failed to identify and convert a Bool type from string.",
                ErrorKind::FailedToIdentify
            ))
        }
    } 
}

impl Object for Bool {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Bool"
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl ScalarValued<bool> for Bool {
    fn value(&self) -> bool {
        self.internal
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

/// Compares `Bool` == `bool`
impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        self.internal == *other
    }
}
/// Compares `bool` == `Bool`.
impl PartialEq<Bool> for bool {
    fn eq(&self, other: &Bool) -> bool {
        *self == other.value()
    }
}

/// Implements `IntoType` for conjugated primitive
impl IntoType for bool {
    type TargetType = Bool;

    fn into_type(&self) -> Self::TargetType {
        Bool {
            internal: *self
        }
    }
}

// Unittest
#[cfg(test)]
#[path = "./unittest/bool/tests.rs"]
mod tests;