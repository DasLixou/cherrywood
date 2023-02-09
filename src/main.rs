use cherrywood::{
    app::App,
    event::{Event, EventKind, OnClick, PointerClick},
    math::point::Point,
    params::{event_catcher::EventCatcher, event_thrower::EventThrower, res::Res, res_mut::ResMut},
    resource::Resource,
    system::IntoDescribedSystem,
    widget::Widget,
    widgets::{button::Button, label::Label, stack::Stack},
};

struct Counter(i32);
impl Resource for Counter {}

fn main() {
    let mut app = App::new(|cx| {
        Stack::new(cx)
            .borrow_mut()
            .with_children(|cx| {
                (
                    Button::new(cx.clone())
                        .borrow_mut()
                        .subscribe_event::<PointerClick, _>(pointer_click.into_described())
                        .subscribe_event::<OnClick, _>((
                            increment_counter.into_described(),
                            send_request.into_described(),
                        ))
                        .finish(),
                    Label::new(cx)
                        .borrow_mut()
                        .with_content(|counter: Res<Counter>| {
                            // TODO: we could notify about change via events
                            println!("Counter changed.");
                            format!("Counter: {}", counter.0)
                        })
                        .finish(),
                )
            })
            .finish()
    });
    app.insert_resource(Counter(0));
    app.queue_events(Event::new(PointerClick(Point(1, 2)), EventKind::Root));
    app.handle();
    app.queue_events(Event::new(PointerClick(Point(2, 0)), EventKind::Root));
    app.handle();
    app.queue_events(Event::new(PointerClick(Point(3, 2)), EventKind::Root));
    app.handle();
}

fn pointer_click(event: EventCatcher<PointerClick>, mut on_click: EventThrower<OnClick>) {
    match event.get().0 {
        Point(0..=2, 0..=5) => {
            let event = event.catch();
            println!("pointer clicked at: {:?}", event.0);
            on_click += (OnClick(event.0.clone()), EventKind::Bubble);
        }
        _ => {}
    }
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Incremented Counter. Is now {}", counter.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
