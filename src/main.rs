use cherrywood::{
    app::App,
    button::Button,
    event::PointerClick,
    label::Label,
    math::point::Point,
    resource::Resource,
    stack::Stack,
    system::IntoDescribedSystem,
    system_param::{Res, ResMut},
};

struct Counter(i32);
impl Resource for Counter {}

fn main() {
    let mut app = App::new(Stack::new().with_children((
        Button::new().subscribe_on_click((
            increment_counter.into_described(),
            send_request.into_described(),
        )),
        Label::new().with_content(|counter: Res<Counter>| {
            println!("Counter changed.");
            format!("Counter: {}", counter.0)
        }),
    )));
    app.insert_resource(Counter(0));
    app.dispatch_event(PointerClick(Point(1, 2)));
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
