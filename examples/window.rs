use ruscii::terminal::{Color, VisualElement, Window};

fn main() {
    let mut window = Window::default();
    window.open();
    println!("This is an open window");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let mut default = VisualElement::default();
    default.background = Color::Red;
    window.canvas_mut().set_default_element(&default);
    window.clear();
    window.draw();
    println!("With a custom background color!");

    std::thread::sleep(std::time::Duration::from_secs(2));
    window.close();
}
