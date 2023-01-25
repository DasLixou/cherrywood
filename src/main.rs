use cherrywood::{button::Button, container::Container};

struct Greeting(String);

fn main() {
    let button = Button::new().on_click(greet);
    let mut container = Container::new(button, Greeting("hello hello :D".to_string()));
    container.click();
    container.click();
}

fn greet(greeting: &Greeting) {
    println!("{}", greeting.0);
}
