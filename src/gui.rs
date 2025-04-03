use gtk::{Button, Label, Window};

pub fn create_main_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTA 5 Online Mod Menu");
    window.set_default_size(400, 300);
    
    let label = Label::new(Some("GTA 5 Mod Menu"));
    let activate_button = Button::new_with_label("Activate Mod");

    window.add(&label);
    window.add(&activate_button);

    window
}