use gtk::{gio::ApplicationFlags, prelude::*};
use crate::visual::wind;

fn on_active(app: &gtk::Application) {
    let main = wind();
    let window = gtk::ApplicationWindow::builder()
        .title("m3u to html")
        .resizable(false)
        .application(app)
        .child(&main)
        .build();
    window.show();
}

pub fn load() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.m3utohtml")
        .build();
    app.connect_activate(on_active);
    app.run();
}
