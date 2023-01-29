use crate::{
    app::App,
    widget::{Widget, WidgetId},
};

pub struct Stack {
    pub children: Vec<Box<dyn Widget>>,
    id: WidgetId,
}

impl Stack {
    pub fn new(app: &mut App) -> &mut Self {
        app.new_widget(|id| Self {
            children: Vec::new(),
            id,
        })
    }

    //pub fn with_children<const N: usize>(f: impl FnOnce(&Widget) -> [])
}

impl Widget for Stack {
    fn id(&self) -> WidgetId {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
