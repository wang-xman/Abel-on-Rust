use crate::object::Object;

pub trait NamedType {
    fn type_name(&self) -> &'static str;
}

pub trait IntoType {
    type TargetType: Object;

    fn into_type(&self) -> Self::TargetType;
}

pub trait Downcast: Object + 'static {
    fn to_ref<TargetAbelType: 'static>(&self) -> Option<&TargetAbelType> {
        self.as_any().downcast_ref::<TargetAbelType>()
    }

    fn to_mut_ref<TargetAbelType: 'static>(&mut self) -> Option<&mut TargetAbelType> {
        self.as_mut_any().downcast_mut::<TargetAbelType>()
    }
}

/// Main use case of trait `Downcast`
impl Downcast for Box<dyn Object> {}