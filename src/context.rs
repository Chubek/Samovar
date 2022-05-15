use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait ContextOp<R, Args> {
    fn op(&self, args: Args) -> R;
}

pub type ContextType = Arc<HashMap<String, Arc<Mutex<dyn Any + Send>>>>;
