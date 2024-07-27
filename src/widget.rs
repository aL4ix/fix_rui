use std::any::Any;

use crate::downcast::Downcast;

pub trait Widget: Any{

    fn class_name() -> &'static str
        where
            Self: Sized;
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize) -> usize;
}

pub struct W1 {
    pub id: usize,
}

impl Widget for W1 {
    fn class_name() -> &'static str where Self: Sized {
        "W1"
    }

    fn id(&self) -> usize {
        self.id
    }
    fn set_id(&mut self, _id: usize) -> usize {
        return self.id;
    }
}

impl Downcast for dyn Widget {

}