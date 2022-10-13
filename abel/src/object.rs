use std::any::Any;

pub trait Object {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn type_name(&self) -> &'static str;
}

impl Object for Box<dyn Object> {
    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self.as_mut().as_mut_any()
    }

    fn type_name(&self) -> &'static str {
        self.as_ref().type_name()
    }
}