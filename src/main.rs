use cherrywood::{
    app::App,
    event::{Event, EventKind, OnClick, PointerClick},
    math::point::Point,
    params::{event_catcher::EventCatcher, event_thrower::EventThrower, res_mut::ResMut},
    resource::Resource,
    widgets::stack::Stack,
};

struct Counter(i32);
impl Resource for Counter {}

fn main() {
    let mut app = App::new(|p, c| Stack::new(p, c).with_children(|_p, _c| {}).finish());
    app.insert_resource(Counter(0));
    app.queue_events(Event::new(PointerClick(Point(1, 2)), EventKind::Root));
    app.handle();
    app.queue_events(Event::new(PointerClick(Point(1, 2)), EventKind::Root));
    app.handle();
}

fn pointer_click(event: EventCatcher<PointerClick>, mut on_click: EventThrower<OnClick>) {
    println!("pointer clicked at: {:?}", event.0);
    on_click += (OnClick(event.0.clone()), EventKind::Bubble);
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
