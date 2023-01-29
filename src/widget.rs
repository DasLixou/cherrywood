use crate::app::App;

pub trait IntoWidget {
    type Widget: Widget;

    fn into_widget(self, app: &mut App) -> Self::Widget;
}

pub trait Widget {}
