use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Box, Orientation};
use glib::clone;

const APP_ID: &str = "org.mygtk4app.gobjectmemmgmt";

// This is just an extension of hello_world project
// for detailed comments on some parts refer that project

fn main() -> glib::ExitCode {

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(a: &Application) {
    
    // Declare a number variable to be incremented and decremented by buttons
    // as Cell type for mutability and Rc for shared ownership
    let number = Rc::new(Cell::new(0));

    // Buttons and other widgets are GObjects which are both reference counting and mutable
    // so no need of Rc<RefCell<T>> type for them

    // Button to increment number variable
    let button_increment = Button::builder()
        .label(String::from("Increment ") + &number.get().to_string())
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Button to decrement number variable
    let button_decrement = Button::builder()
        .label(String::from("Decrement ") + &number.get().to_string())
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect handlers for click on both buttons
    // We use the glib::clone! macro to have strong and weak references
    // without needing to create a copy of number everytime and do Rc::clone()
    button_increment.connect_clicked(clone!(@weak button_decrement, @weak button_increment, @strong number =>
        move |_| {
            number.set(number.get() + 1);
            button_decrement.set_label(&(String::from("Decrement ") + &number.get().to_string()));
            button_increment.set_label(&(String::from("Increment ") + &number.get().to_string()));
        }));

        // Need to keep number alive so moving ownership to button_decrement
        // So, need to use nothing for number here
        button_decrement.connect_clicked(clone!(@weak button_increment, @weak button_decrement =>
            move |_| {
                number.set(number.get() - 1);
                button_decrement.set_label(&(String::from("Decrement ") + &number.get().to_string()));
                button_increment.set_label(&(String::from("Increment ") + &number.get().to_string()));
            }));

    // Box to contain both buttons in vertical orientation
    let box_container = Box::builder()
              .orientation(Orientation::Vertical)
              .build();

    // Add buttons to box_container
    box_container.append(&button_increment);
    box_container.append(&button_decrement);         

    // Make Box a child of the window
    let window = ApplicationWindow::builder()
        .application(a)
        .title("GObject Memory Management")
        .child(&box_container)
        .build();

    // Present the window
    window.present();
}