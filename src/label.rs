use crate::{
    app::App,
    system::{BoxedDescribedSystem, DescribedSystem, IntoDescribedSystem},
    system_param::SystemParam,
    widget::{Widget, WidgetId},
};

pub struct Label {
    pub content: Option<BoxedDescribedSystem<String>>,
    id: WidgetId,
}

impl Label {
    pub fn new(app: &mut App) -> &mut Self {
        app.new_widget(|id| Self { content: None, id })
    }

    pub fn with_content<F: IntoDescribedSystem<String, Params>, Params: SystemParam>(
        mut self,
        system: F,
    ) -> Self {
        let mut system = Box::new(system.into_described());
        system.initialize();
        self.content = Some(system);
        self
    }
}

impl Widget for Label {
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
