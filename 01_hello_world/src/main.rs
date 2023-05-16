use gtk::prelude::*; // for importing necessary traits
use gtk::{glib, Application, ApplicationWindow, Button};

// Need a unique application id for the application
const APP_ID: &str = "org.mygtk4app.HelloWorld";

fn main() -> glib::ExitCode {
    // Create new Application using builder pattern (https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of Application by specifying build_ui() as the handler
    app.connect_activate(build_ui);

    // Run the Application
    app.run()
}

// the handler for "activate" signal
fn build_ui(app: &Application) {
    // Create a button
    let button = Button::builder()
        .label("Click Me :)")
        .margin_bottom(12)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // connect button to a click handler by using a closure
    button.connect_clicked(|button| {
        button.set_label("Hello World :)")
    } );

    // Create a window and set it's title and set the button as it's child
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My HelloWorld GTK4 App")
        .child(&button)
        .build();       

    // Present the window()
    window.present()        
}