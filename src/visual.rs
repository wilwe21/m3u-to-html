use gtk::prelude::*;

pub fn wind() -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let lab = gtk::Label::new(Some("test"));
    let button = gtk::Button::builder()
        .label("sas")
        .build();
    let BOX = gtk::Box::builder()
        .child(&lab)
        .child(&button)
        .build();
    return BOX;
}
