use std::cell::RefCell;
use std::rc::Rc;

use crate::downcast::Downcast;
use crate::root::Root;
use crate::widget::{W1, Widget};
use crate::widget_manager::WidgetEnum;
use crate::WidgetIds::{Ew1, Ew2};
use crate::window_builder::WindowBuilder;

mod root;
mod widget;
mod downcast;
mod window_builder;
mod widget_manager;

#[derive(Copy, Clone)]
enum WidgetIds {
    Ew1 = -1,
    Ew2 = 2
}

impl WidgetEnum for WidgetIds {
    fn to_isize(self) -> isize {
        self as isize
    }
}

fn main() {
    println!("Hello, world!");
    let mut win = WindowBuilder::new();
    win.add_widget(Rc::new(RefCell::new(W1 { id: 1 })), Ew1, 0);
    win.add_widget(Rc::new(RefCell::new(W1 { id: 1 })), Ew2, 1);

    println!("a");
    let option1 = win.get_widget_by_id(Ew1);

    println!("b");
    let my_w = option1.unwrap();
    println!("c");
    let mut refmut = my_w.borrow_mut();
    let w1 = refmut.downcast_mut::<W1>().unwrap();
    println!("{}", w1.id());
    w1.set_id(0);
    let option2 = win.get_widget_by_id2::<W1>(Ew2);
    println!("j");
    let _w2 = option2.unwrap();
    println!("next");





}
