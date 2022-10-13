use std::fmt;
use std::any::Any;
use std::ops::Deref;

use crate::object::Object;
use crate::marker::ScalarValued;
use crate::typefy::IntoType;

#[derive(Debug, PartialEq)]
pub struct Text {
    internal: String,
}

impl Text {
    pub fn new(value: &str) -> Text {
        Text {
            internal: value.to_owned()
        }
    }

    pub fn from(value: &str) -> Text {
        Text {
            internal: value.to_owned()
        }
    }
}

impl Object for Text {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Text"
    }
}

impl ScalarValued<String> for Text {
    fn value(&self) -> String {
        self.internal.clone()
    }
}

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.internal)
    }
}

// Compares `Text == String`
impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        self.internal == *other
    }
}
// Compares `Text == &String`
impl PartialEq<&String> for Text {
    fn eq(&self, other: &&String) -> bool {
        self.internal == **other
    }
}
// Compares `Text == &str`
impl PartialEq<&str> for Text {
    fn eq(&self, other: &&str) -> bool {
        self.internal == *other
    }
}
// Compares `Text == str`
impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.internal == *other
    }
}

// Compares `String == Text`
impl PartialEq<Text> for String {
    fn eq(&self, other: &Text) -> bool {
        *self == *other.value()
    }
}
// Compares `&str == Text`
impl PartialEq<Text> for &str {
    fn eq(&self, other: &Text) -> bool {
        *self == other.value()
    }
}

impl IntoType for &str {
    type TargetType = Text;

    fn into_type(&self) -> Self::TargetType {
        Text {
            internal: self.to_string()
        }
    }
}

impl IntoType for String {
    type TargetType = Text;

    fn into_type(&self) -> Self::TargetType {
        Text {
            internal: self.to_string()
        }
    }
}

// Unittest
#[cfg(test)]
#[path = "./unittest/text/tests.rs"]
mod tests;