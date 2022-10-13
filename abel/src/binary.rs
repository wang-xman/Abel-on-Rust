//! THINK! Do I need to implement IntoType for Rust primitives?
use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::object::Object;
//use crate::typefy::IntoType;
use crate::marker::ScalarValued;
use crate::util;

#[derive(Debug, PartialEq)]
pub struct Binary {
    internal: String,
}

impl Binary {
    pub fn new(internal: &str) -> Self {
        if !util::is_valid_binary_string(internal) {
            panic!("String {} is not a valid binary string", internal);
        }
        Binary {
            internal: internal.to_owned()
        }
    }

    pub fn from(internal: &str) -> Self {
        if !util::is_valid_binary_string(internal) {
            panic!("String {} is not a valid binary string", internal);
        }
        Binary {
            internal: internal.to_owned()
        }
    }
}

impl Object for Binary {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Binary"
    }
}

impl ScalarValued<String> for Binary {
    fn value(&self) -> String {
        self.internal.clone()
    }
}

impl Deref for Binary {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

// unittest
#[cfg(test)]
#[path = "./unittest/binary/tests.rs"]
mod tests;
