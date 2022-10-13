use crate::error::InternalError;
use crate::object::Object;
use crate::typefy::IntoType;
use crate::marker::ScalarLikePrimitive;

pub trait Container<KeyType> {
    fn get_type_name(&self, k: KeyType) -> Result<&'static str, InternalError>;

    fn get_ref<TargetType>(&self, k: KeyType)
    -> Result<&TargetType, InternalError>
    where
        TargetType: 'static + Object;

    fn get_mut_ref<TargetType>(&mut self, k: KeyType)
    -> Result<&mut TargetType, InternalError>
    where
        TargetType: 'static + Object;

    fn set<SourceType>(&mut self, k: KeyType, obj: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + Object;

    fn set_box(&mut self, k: KeyType, obj_boxed: Box<dyn Object>)
    -> Result<&mut Self, InternalError>;

    fn set_from<SourceType>(&mut self, k: KeyType, prim_type: SourceType)
    -> Result<&mut Self, InternalError>
    where
        SourceType: 'static + IntoType + ScalarLikePrimitive;
}