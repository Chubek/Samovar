use std::sync::{Mutex, Arc};
use std::any::Any;
use std::ops::Deref;

pub struct Context {
    item: Arc<Mutex<&'static dyn Any>>,
}

impl Context {
    pub fn new(item: &'static dyn Any) -> Self {
        let arc_mutex = Arc::new(Mutex::new(item));

        Context { item:arc_mutex }
    }

    pub fn get_item_ref(&self) -> &Mutex<&'static dyn Any> {
        let deref = self.item.deref();

        deref
    }
}