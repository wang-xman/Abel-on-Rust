use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::object::Object;
use crate::typefy::IntoType;
use crate::marker::ScalarValued;

#[derive(Debug, PartialEq)]
pub struct Integer {
    internal: i32,
}

impl Integer {
    pub fn new(internal: i32) -> Self {
        Integer {
            internal
        }
    }

    pub fn from(internal: i32) -> Self {
        Integer {
            internal
        }
    }
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Integer"
    }
}

impl ScalarValued<i32> for Integer {
    fn value(&self) -> i32 {
        self.internal
    }
}

impl Deref for Integer {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}


impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

/// compares `Integer == i32`
impl PartialEq<i32> for Integer {
    fn eq(&self, other: &i32) -> bool {
        self.internal == *other
    }
}
/// compares `Integer == f64`
impl PartialEq<f64> for Integer {
    fn eq(&self, other: &f64) -> bool {
        (self.internal as f64) == *other
    }
}
/// compares `i32 == Integer`
impl PartialEq<Integer> for i32 {
    fn eq(&self, other: &Integer) -> bool {
        *self == other.value()
    }
}
/// compares `f64 == Integer`
impl PartialEq<Integer> for f64 {
    fn eq(&self, other: &Integer) -> bool {
        *self == (other.value() as f64)
    }
}

impl IntoType for i32 {
    type TargetType = Integer;

    fn into_type(&self) -> Self::TargetType {
        Integer {
            internal: *self
        }
    }
}

// unittest
#[cfg(test)]
#[path = "./unittest/integer/tests.rs"]
mod tests;