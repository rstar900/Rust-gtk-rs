// imp module will handle the state and overridden GObject virtual functions

use std::cell::Cell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// This object will hold state
#[derive(Default)]
pub struct CustomButton{
    number : Cell<i32>
}

// The main trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtk4AppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// The trait shared by all GObjects
impl ObjectImpl for CustomButton {
    // We want to set the label to be the initial number value on construction of CustomButton Object
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.number.get().to_string());
    }
}

// The trait shared for all widgets
impl WidgetImpl for CustomButton {}

// The trait shared for all Buttons
impl ButtonImpl for CustomButton {
    // We want to increment number on click
    fn clicked(&self) {
        self.number.set(self.number.get() + 1);
        self.obj().set_label(&self.number.get().to_string());
    }
}