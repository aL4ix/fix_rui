use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use crate::widget::Widget;


pub type MyWidget = Rc<RefCell<dyn Widget>>;

pub trait WidgetEnum: Clone + Copy {
    fn to_isize(self) -> isize;
}


#[derive(Default)]
pub struct WidgetManager {
    widgets: BTreeMap<isize, MyWidget>,
}


impl WidgetManager {
    pub fn insert(&mut self, id: impl WidgetEnum, widget: MyWidget) {
        self.widgets.insert(id.to_isize(), widget);
    }

    pub fn get(&self, id: impl WidgetEnum) -> Option<MyWidget> {
        self.widgets.get(&id.to_isize()).cloned()
    }
}