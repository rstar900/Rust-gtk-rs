// custom_button module will handle exposed methods from the implemented traits and added ones

mod imp;

use gtk::glib;
use glib::Object;

// look for the the implemented traits and inheritance hierarchy on https://docs.gtk.org/gtk4/class.Button.html#hierarchy
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

// Exposed methods
impl CustomButton {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
}

// Default trait
impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}