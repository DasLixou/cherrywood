use cherrywood::{
    app::App,
    button::Button,
    label::Label,
    resource::Resource,
    stack::Stack,
    system_param::{Res, ResMut},
    widget::Widget,
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

fn ui() -> impl Widget {
    let mut button = Button::new();
    button.on_click.subscribe(increment_counter);
    button.on_click.subscribe(send_request);

    let _label = Label::new().with_content(|counter: Res<Counter>| {
        println!("Counter changed.");
        format!("Counter: {}", counter.0)
    });

    Stack::new().with_children(button)
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
