use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::object::Object;
use crate::typefy::IntoType;
use crate::marker::ScalarValued;
use crate::integer::Integer;

#[derive(Debug, PartialEq)]
pub struct Double {
    internal: f64,
}

impl Double {
    pub fn new(internal: f64) -> Self {
        Double {
            internal
        }
    }

    pub fn from(internal: f64) -> Self {
        Double {
            internal
        }
    }
}

impl Object for Double {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Double"
    }
}

impl ScalarValued<f64> for Double {
    fn value(&self) -> f64 {
        self.internal
    }
}

impl Deref for Double {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

/// Double == Integer
impl PartialEq<Integer> for Double {
    fn eq(&self, other: &Integer) -> bool {
        self.internal == (other.value() as f64)
    }
}
/// Double == f64
impl PartialEq<f64> for Double {
    fn eq(&self, other: &f64) -> bool {
        self.internal == *other
    }
}
/// Double == i32 (double)
impl PartialEq<i32> for Double {
    fn eq(&self, other: &i32) -> bool {
        self.internal == (*other as f64)
    }
} 
/// Integer == Double
impl PartialEq<Double> for Integer {
    fn eq(&self, other: &Double) -> bool { 
        self.value() as f64 == other.value()
    }
}
/// f64 == Double
impl PartialEq<Double> for f64 {
    fn eq(&self, other: &Double) -> bool {
        *self == other.value()
    }
}
/// i32 == Double
impl PartialEq<Double> for i32 {
    fn eq(&self, other: &Double) -> bool {
        (*self as f64) == other.value()
    }
}

impl IntoType for f64 {
    type TargetType = Double;

    fn into_type(&self) -> Self::TargetType {
        Double {
            internal: *self
        }
    }
}

// Unittest
#[cfg(test)]
#[path = "./unittest/double/tests.rs"]
mod tests;