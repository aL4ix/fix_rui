use crate::widget_manager::{MyWidget, WidgetEnum};

pub trait Root {
    fn get_widget_by_id(&mut self, id: impl WidgetEnum) -> Option<MyWidget>;
}