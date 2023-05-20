mod custom_button;
use custom_button::CustomButton;

use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow};

const APP_ID: &str = "org.mygtk4app.gobjectsubclassing";

fn main() -> glib::ExitCode {
    let app = Application::builder()
    .application_id(APP_ID)
    .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(a: &Application) {
    // Instantiate our CustomButton object
    let my_custom_button = CustomButton::new(); // this will call the constructed function setting label as 0
    my_custom_button.set_margin_top(12);
    my_custom_button.set_margin_bottom(12);
    my_custom_button.set_margin_start(12);
    my_custom_button.set_margin_end(12);

    let window = ApplicationWindow::builder()
        .application(a)
        .child(&my_custom_button)
        .title("GObject Subclassing Example")
        .build();

    window.present();
}
