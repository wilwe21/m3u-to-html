use gtk::prelude::*;

use crate::main;

pub fn wind() -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let lab = gtk::Label::new(Some("test"));
    let button = gtk::Button::builder()
        .label("sas")
        .build();
    mainBox.child_expands(&button);
    mainBox.child_expands(&lab);
    return mainBox;
}
