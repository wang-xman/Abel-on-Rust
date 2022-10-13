pub trait ScalarValued<T> {
    fn value(&self) -> T;
}

pub trait ScalarLikePrimitive {}

pub trait VectorLikePrimitive {}

impl ScalarLikePrimitive for i32 {}

impl ScalarLikePrimitive for bool {}

impl ScalarLikePrimitive for f64 {}

impl ScalarLikePrimitive for &str {}

impl ScalarLikePrimitive for String {}