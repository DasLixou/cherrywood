use cherrywood::{
    app::App,
    button::Button,
    event::PointerClick,
    label::Label,
    math::point::Point,
    params::{event_catcher::EventCatcher, res::Res, res_mut::ResMut},
    resource::Resource,
    stack::Stack,
    system::IntoDescribedSystem,
};

struct Counter(i32);
impl Resource for Counter {}

fn main() {
    let mut app = App::new(Stack::new().with_children((
        Button::new().subscribe_event::<PointerClick, _>((
            pointer_click.into_described(),
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
    app.dispatch_event(PointerClick(Point(1, 2)));
}

fn pointer_click(event: EventCatcher<PointerClick>) {
    println!("pointer clicked at: {:?}", event.0);
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
