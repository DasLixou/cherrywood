use cherrywood::{button::Button, container::Container, resource::Resource, system_param::Res};

struct Greeting(String);
impl Resource for Greeting {}

fn main() {
    let mut button = Button::new();
    button.on_click.subscribe(greet);
    button.on_click.subscribe(send_request);
    let mut container = Container::new();
    let greeting = Greeting("hello hello :D".to_string());
    container.insert_resource(greeting);
    button.on_click.run(&mut container);
}

fn greet(greeting: Res<Greeting>) {
    println!("{}", greeting.0);
}

fn send_request() {
    println!("Attempting to send server request..");
}
