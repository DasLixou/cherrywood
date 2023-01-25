use cherrywood::{button::Button, container::Container, resource::Resource};

struct Greeting(String);
impl Resource for Greeting {}

fn main() {
    let button = Button::new().on_click(greet);
    let mut container = Container::new(button);
    container.insert_resource(Greeting("hello hello :D".to_string()));
    container.click();
    container.click();
}

fn greet(/*greeting: &Greeting*/) {
    // TODO: make params work again
    //println!("{}", greeting.0);
}
