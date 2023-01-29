use cherrywood::{
    app::App,
    button::Button,
    resource::Resource,
    system_param::ResMut,
    widget::{Widget, WidgetId},
};

struct Counter(i32);
impl Resource for Counter {}

fn main() {
    /*let mut button = Button::new();
    button.on_click.subscribe(increment_counter);
    button.on_click.subscribe(send_request);

    let label = Label::new().with_content(|counter: Res<Counter>| {
        println!("Counter changed.");
        format!("Counter: {}", counter.0)
    });*/

    let mut app = App::new(ui);
    app.insert_resource(Counter(0));

    /*button.on_click.run(&mut app);

    black_box(label);*/
}

fn ui(app: &mut App) -> WidgetId {
    let button = Button::new(app);
    button.on_click.subscribe(increment_counter);
    button.on_click.subscribe(send_request);
    button.id()
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
