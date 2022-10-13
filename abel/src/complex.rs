use std::fmt;
use std::any::Any;

use crate::object::Object;
use crate::marker::ScalarValued;
use crate::integer::Integer;
use crate::double::Double;

#[derive(Debug, PartialEq)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex {
            real: real,
            imag: imag
        }
    }

    pub fn from_f64(real: f64) -> Self {
        Complex {
            real: real,
            imag: 0.0
        }
    }

    pub fn from_i32(real: i32) -> Self {
        Complex {
            real: real as f64,
            imag: 0.0
        }
    }

    pub fn value(&self) -> (f64, f64) {
        (self.real, self.imag)
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }

    pub fn is_real(&self) -> bool {
        self.imag == 0.0
    }

    pub fn is_imag(&self) -> bool {
        self.real == 0.0 && self.imag != 0.0
    }

    pub fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imag == 0.0
    }
}

impl Object for Complex {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Complex"
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.real, self.imag)
    }
}

/// Complex == Double
impl PartialEq<Double> for Complex {
    fn eq(&self, other: &Double) -> bool {
        self.imag() == 0.0 && self.real() == other.value()
    }
}
/// Complex == Integer
impl PartialEq<Integer> for Complex {
    fn eq(&self, other: &Integer) -> bool {
        self.imag() == 0.0 && self.real() == (other.value() as f64)
    }
}
/// Complex == f64
impl PartialEq<f64> for Complex {
    fn eq(&self, other: &f64) -> bool {
        self.imag() == 0.0 && self.real() == *other
    }
}
/// Complex == i32
impl PartialEq<i32> for Complex {
    fn eq(&self, other: &i32) -> bool {
        self.imag() == 0.0 && self.real() == *other as f64
    }
}
/// Double == Complex
impl PartialEq<Complex> for Double {
    fn eq(&self, other: &Complex) -> bool {
        other.imag() == 0.0 && other.real() == self.value()
    }
}
/// Integer == Complex
impl PartialEq<Complex> for Integer {
    fn eq(&self, other: &Complex) -> bool {
        other.imag() == 0.0 && other.real() == self.value() as f64
    }
}
/// f64 == Complex
impl PartialEq<Complex> for f64 {
    fn eq(&self, other: &Complex) -> bool {
        other.imag() == 0.0 && other.real() == *self
    }
}
/// i32 == Complex
impl PartialEq<Complex> for i32 {
    fn eq(&self, other: &Complex) -> bool {
        other.imag() == 0.0 && other.real() == *self as f64
    }
}

#[cfg(test)]
#[path = "./unittest/complex/tests.rs"]
mod tests;