use std::any::Any;

pub struct Context {
    item: &'static (dyn Any + Sync),
}

impl Context {
    pub fn new(item: &'static  (dyn Any + Sync)) -> Self {
        Context { item }
    } 
}