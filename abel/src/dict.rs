use std::any::Any;
use std::collections::HashMap;

use crate::error::{Error, InternalError, ErrorKind};
use crate::object::Object;
use crate::marker::ScalarLikePrimitive;
use crate::typefy::IntoType;
use crate::container::Container;

pub struct Dict {
    internal: HashMap<String, Box<dyn Object>>,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            internal: HashMap::<String, Box<dyn Object>>::new()
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.internal.contains_key(key)
    }

    pub fn insert<T>(&mut self, key: &str, obj: T)
    where
        T: 'static + Object
    {
        self.internal.insert(key.to_owned(), Box::new(obj));
    }

    pub fn insert_box(&mut self, key: &str, obj_boxed: Box<dyn Object>) {
        self.internal.insert(key.to_owned(), obj_boxed);
    }

    pub fn insert_from<T>(&mut self, key: &str, prim_type: T)
    where
        T: 'static + IntoType + ScalarLikePrimitive
    {
        self.internal.insert(key.to_owned(), Box::new(prim_type.into_type()));
    }
    
    fn get(&self, k: &str) -> Option<&Box<dyn Object>> {
        self.internal.get(k)
    }

    fn get_mut(&mut self, k: &str) -> Option<&mut Box<dyn Object>> {
        self.internal.get_mut(k)
    }
}

impl Object for Dict {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        "Dict"
    }
}

impl Container<&str> for Dict {
    fn get_type_name(&self, key: &str) -> Result<&'static str, InternalError> {
        if self.has_key(key) {
            Ok(self.get(key).unwrap().as_ref().type_name())
        } else {
            let msg = format!("Key \"{}\" not found in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::KeyNotFound))
        }
    }

    fn get_ref<TargetType>(&self, key: &str)
    -> Result<&TargetType, InternalError>
    where
        TargetType: 'static + Object
    {
        if self.has_key(key) {
            if self.get(key).unwrap().as_ref().as_any().is::<TargetType>() {
                Ok(self.get(key).unwrap().as_ref().as_any()
                       .downcast_ref::<TargetType>().unwrap())
            } else {
                let msg = format!("Value at key \"{}\" is of type {}, failed \
                                  to match the requested type {}.",
                                  key, self.get(key).unwrap().as_ref().type_name(),
                                  std::any::type_name::<TargetType>());
                Err(InternalError::new(&msg, ErrorKind::MismatchedType))
            }
        } else {
            let msg = format!("Key \"{}\" not found in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::KeyNotFound))
        }
    }

    fn get_mut_ref<TargetType>(&mut self, key: &str)
    -> Result<&mut TargetType, InternalError>
    where
        TargetType: 'static + Object
    {
        if self.has_key(key) {
            if self.get(key).unwrap().as_ref().as_any().is::<TargetType>() {
                Ok(self.get_mut(key).unwrap().as_mut().as_mut_any()
                       .downcast_mut::<TargetType>().unwrap())
            } else {
                let msg = format!("Value at key \"{}\" is of type {}, failed \
                                  to match the request type {}.",
                                  key, self.get(key).unwrap().as_ref().type_name(),
                                  std::any::type_name::<TargetType>());
                Err(InternalError::new(&msg, ErrorKind::MismatchedType))
            }
        } else {
            let msg = format!("Key \"{}\" not found in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::KeyNotFound))
        }
    }

    fn set<SourceType>(&mut self, key: &str, obj: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + Object
    {
        if self.has_key(key) {
            let msg = format!("Key \"{}\" already exists in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::DuplicateKey))
        } else {
            self.internal.insert(key.to_owned(), Box::new(obj));
            Ok(self)
        }
    }

    fn set_box(&mut self, key: &str, obj_boxed: Box<dyn Object>)
    -> Result<&mut Self, InternalError>
    {
        if self.has_key(key) {
            let msg = format!("Key \"{}\" already exists in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::DuplicateKey))
        } else {
            self.insert_box(key, obj_boxed);
            Ok(self)
        }
    }

    fn set_from<SourceType>(&mut self, key: &str, prim_type: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + IntoType + ScalarLikePrimitive
    {
        if self.has_key(key) {
            let msg = format!("Key \"{}\" already exists in dictionary.", key);
            Err(InternalError::new(&msg, ErrorKind::DuplicateKey))
        } else {
            self.insert_from(key, prim_type);
            Ok(self)
        }
    }
}

#[cfg(test)]
#[path = "./unittest/dict/tests.rs"]
mod tests;