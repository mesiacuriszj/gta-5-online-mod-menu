use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::process::Command;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTA 5 Online Mod Menu");
    window.set_default_size(300, 200);

    let label = Label::new(Some("Welcome to GTA 5 Online Mod Menu"));
    let button = Button::new_with_label("Activate Mod");

    button.connect_clicked(|_| {
        Command::new("gta5_mod_script.exe").spawn().expect("Failed to execute mod script.");
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}