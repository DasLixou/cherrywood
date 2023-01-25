use cherrywood_widgets::button::Button;

fn main() {
    println!("Hello, world!");
    let mut button = Button::new().on_click(greet);
    button.click();
    button.click();
}

fn greet() {
    println!("hello 👋");
}
