use std::any::Any;
use std::ops::{Index, IndexMut};
use std::ops::Deref;

use crate::error::{Error, InternalError, ErrorKind};
use crate::object::Object;
use crate::marker::ScalarLikePrimitive;
use crate::typefy::IntoType;
use crate::container::Container;

pub struct List {
    internal: Vec<Box<dyn Object>>,
}

impl List {
    pub fn new() -> List {
        List {
            internal: Vec::<Box<dyn Object>>::new()
        }
    }

    pub fn from_slice<T>(prim_slice: &[T]) -> List
    where
        T: 'static + IntoType + ScalarLikePrimitive
    {
        let mut vec = Vec::<Box<dyn Object>>::new();
        for item in prim_slice {
            vec.push(Box::new(item.into_type()));
        }
        List {
            internal: vec
        }
    }
    
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    pub fn push<T>(&mut self, obj: T)
    where
        T: 'static + Object
    {
        self.internal.push(Box::new(obj));
    }

    pub fn push_box(&mut self, obj_boxed: Box<dyn Object>) {
        self.internal.push(obj_boxed);
    }

    pub fn push_from<T>(&mut self, prim_type: T)
    where
        T: 'static + IntoType + ScalarLikePrimitive
    {
        self.internal.push(Box::new(prim_type.into_type()));
    }

    pub fn push_from_slice<T>(&mut self, prim_slice: &[T])
    where
        T: 'static + Copy + IntoType + ScalarLikePrimitive
    {
        // box each element and push it into the list
        for &item in prim_slice {
            self.internal.push(Box::new(item.into_type()));
        }
    }

    pub fn insert<T>(&mut self, index: usize, obj: T)
    where
        T: 'static + Object
    {
        if index > self.len() {
            panic!("List insertion index {} is out of range", index);
        } else {
            self.internal.insert(index, Box::new(obj));
        }
    }

    pub fn insert_box(&mut self, index: usize, obj: Box<dyn Object>) {
        if index > self.len() {
            panic!("List insertion index {} is out of range", index);
        } else {
            self.internal.insert(index, obj);
        }
    }

    pub fn insert_from<T>(&mut self, index: usize, prim_type: T)
    where
        T: 'static + IntoType + ScalarLikePrimitive
    {
        if index > self.len() {
            panic!("List insertion index {} is out of range", index);
        } else {
            self.internal.insert(index, Box::new(prim_type.into_type()));
        }
    }
}

impl Object for List {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "List"
    }
}

/// Specialise `Container` trait for List, setting key type to `usize`
impl Container<usize> for List {
    fn get_type_name(&self, index: usize)
    -> Result<&'static str, InternalError>
    {
        if index < self.len() {
            Ok(self.index(index).as_ref().type_name())
        } else {
            let msg = format!("Requested index {} is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        }
    }

    fn get_ref<TargetType>(&self, index: usize)
    -> Result<&TargetType, InternalError>
    where
        TargetType: 'static + Object
    {
        if index < self.len() {
            if self.index(index).as_ref().as_any().is::<TargetType>() {
                Ok(self.index(index).as_any().downcast_ref::<TargetType>().unwrap())
            } else {
                let msg = format!("Element at index {} is of type {}, \
                        failed to match the requested type {}.",
                        index,
                        self.index(index).as_ref().type_name(),
                        std::any::type_name::<TargetType>());
                Err(InternalError::new(&msg, ErrorKind::MismatchedType))
            }
        } else {
            let msg = format!("Requested index {} is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        }
    }
    
    // TODO 1 and 2
    fn get_mut_ref<TargetType>(&mut self, index: usize)
    -> Result<&mut TargetType, InternalError>
    where
        TargetType: 'static + Object
    {
        if index < self.len() {
            if self.index_mut(index).as_mut_any().is::<TargetType>() {
                Ok(self.index_mut(index).as_mut_any()
                       .downcast_mut::<TargetType>().unwrap())    
            } else {
                let msg = format!("Element at index {} is of type {}, failed \
                        to match the requested type {}.",
                        index,
                        self.index(index).as_ref().type_name(),
                        std::any::type_name::<TargetType>());
                Err(InternalError::new(&msg, ErrorKind::SyntaxError))
            }
        } else {
            let msg = format!("Requested index {} is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        }
    }

    fn set<SourceType>(&mut self, index: usize, obj: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + Object
    {
        if index > self.len() {
            let msg = format!("Index {} for setting/inserting \
                    an item in list is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        } else {
            self.insert(index, obj);
            Ok(self)
        }
    }

    fn set_box(&mut self, index: usize, obj_boxed: Box<dyn Object>)
    -> Result<&mut Self, InternalError>
    {
        if index > self.len() {
            let msg = format!("Index {} for setting/inserting \
                    a boxed item in list is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        } else {
            self.insert_box(index, obj_boxed);
            Ok(self)
        }
    }

    fn set_from<SourceType>(&mut self, index: usize, prim_type: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + IntoType + ScalarLikePrimitive
    {
        if index > self.len() {
            let msg = format!("Index {} for setting/inserting \
                    an item in list is out of range.", index);
            Err(InternalError::new(&msg, ErrorKind::IndexOutOfRange))
        } else {
            self.insert_from(index, prim_type);
            Ok(self)
        }
    }
}

impl Deref for List {
    type Target = Vec<Box<dyn Object>>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl Index<usize> for List {
    type Output = Box<dyn Object>;
    
    fn index(&self, i: usize) -> &Self::Output {
        &self.internal[i]
    }
}

impl IndexMut<usize> for List {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.internal[i]
    }
}

#[cfg(test)]
#[path = "./unittest/list/tests.rs"]
mod tests;