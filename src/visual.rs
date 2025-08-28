use gtk::prelude::*;

pub fn wind() -> gtk::Box {
    let mut mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let lab = gtk::Label::new(Some("test"));
    mainBox.set_child(Some(&lab));
    return mainBox;
}
