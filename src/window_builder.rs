use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use crate::downcast::Downcast;

use crate::root::Root;
use crate::widget_manager::{MyWidget, WidgetEnum, WidgetManager};

pub struct WindowBuilder {
    widget_to_render: BTreeMap<usize, isize>,
    widget_man: WidgetManager,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder{
        WindowBuilder {
            widget_to_render: Default::default(),
            widget_man: Default::default(),
        }
    }
    pub fn add_widget(&mut self, widget: MyWidget, widget_id: impl WidgetEnum, render_id: usize) {
        self.widget_man.insert(widget_id, widget);
        self.widget_to_render.insert(render_id, widget_id.to_isize());
    }
    pub fn get_widget_by_id2<T: 'static>(&mut self, id: impl WidgetEnum) -> Option<Rc<RefCell<T>>> {
        for (_, wid_id) in &self.widget_to_render {
            if *wid_id == id.to_isize() {
                println!("get_widget_by_id a");
                let a = self.widget_man.get(id);
                match a {
                    None => {return None}
                    Some(b) => {
                        let mut c = b.borrow_mut();
                        let d = c.downcast_mut::<T>();
                        match d {
                            None => {return None}
                            Some(e) => {return Some(b as Rc<RefCell<T>>)}
                        }
                    }
                }
            }
        }
        // self.widgets.values_mut().find(|widget| widget.borrow().id() == id)

        return None;
    }
}

impl Root for WindowBuilder {
    fn get_widget_by_id(&mut self, id: impl WidgetEnum) -> Option<MyWidget> {
        for (_, wid_id) in &self.widget_to_render {
            if *wid_id == id.to_isize() {
                println!("get_widget_by_id a");
                return self.widget_man.get(id)
            }
        }
        // self.widgets.values_mut().find(|widget| widget.borrow().id() == id)

        return None;
    }
}