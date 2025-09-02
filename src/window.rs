use gtk::prelude::*;
use gtk::gdk;
use crate::visual::wind;

fn on_active(app: &gtk::Application) {
    let mainBox = wind();
    let window = gtk::ApplicationWindow::builder()
        .title("m3u to html")
        .application(app)
        .build();
    conf_css();
    window.set_child(Some(&mainBox));
    window.show();
}

pub fn load() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.m3utohtml")
        .build();
    app.connect_activate(on_active);
    app.run();
}

pub fn conf_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    let css_content = include_str!("./css/main.css");
    provider.load_from_data(&css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}
