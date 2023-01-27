use cherrywood::{button::Button, container::Container, resource::Resource, system_param::Res};

struct Greeting(String);
impl Resource for Greeting {}

fn main() {
    let button = Button::new().on_click(greet);
    let mut container = Container::new(button);
    container.insert_resource(Greeting("hello hello :D".to_string()));
    container.click();
    container.click();
}

fn greet(greeting: Res<Greeting>) {
    println!("{}", greeting.0);
}
