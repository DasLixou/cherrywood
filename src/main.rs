use cherrywood::{button::Button, container::Container, resource::Resource, system_param::Res};

struct Greeting(String);
impl Resource for Greeting {}

fn main() {
    let mut button = Button::new().on_click(greet);
    let mut container = Container::new();
    container.insert_resource(Greeting("hello hello :D".to_string()));
    if let Some(f) = &mut button.on_click {
        f.run(&mut container);
    }
}

fn greet(greeting: Res<Greeting>) {
    println!("{}", greeting.0);
}
