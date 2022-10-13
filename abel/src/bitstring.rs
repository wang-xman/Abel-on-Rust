//! THINK! Do I need to implement IntoType for Rust primitives?
use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::object::Object;
//use crate::typefy::IntoType;
use crate::marker::ScalarValued;
use crate::util;

#[derive(Debug, PartialEq)]
pub struct Bitstring {
    internal: String,
}

impl Bitstring{
    pub fn new(internal: &str) -> Self {
        if !util::is_valid_bitstring(internal) {
            panic!("String {} is not a valid Bitstring", internal);
        }
        Bitstring{
            internal: internal.to_owned()
        }
    }

    pub fn from(internal: &str) -> Self {
        if !util::is_valid_bitstring(internal) {
            panic!("String {} is not a valid Bitstring", internal);
        }
        Bitstring{
            internal: internal.to_owned()
        }
    }
}

impl Object for Bitstring{
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

impl ScalarValued<String> for Bitstring{
    fn value(&self) -> String {
        self.internal.clone()
    }
}

impl Deref for Bitstring{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl fmt::Display for Bitstring{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

// unittest
#[cfg(test)]
#[path = "./unittest/bitstring/tests.rs"]
mod tests;
