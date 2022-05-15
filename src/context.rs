use std::{any::Any, sync::{Mutex, Arc}, collections::HashMap, pin::Pin, rc::Rc};


pub trait ContextOp<R, Args> {
    fn op(&self, args: Args) -> R;
}


pub type ContextType = Arc<HashMap<String, Arc<Mutex<dyn Any + Send>>>>;