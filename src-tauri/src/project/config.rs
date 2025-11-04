use std::any::Any;

pub trait Config: Any {
    fn as_any(&self) -> &dyn Any;
}
