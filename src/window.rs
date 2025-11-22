use std::path::Path;

use dirs::config_dir;
use gtk::prelude::*;
use gtk::gdk;
use crate::parser;
use crate::visual::wind;

pub const id: &str = "com.github.wilwe21.m3utohtml";

fn on_active(app: &gtk::Application) {
    let mainBox = wind(&app.clone());
    let window = gtk::ApplicationWindow::builder()
        .title("m3u to html")
        .application(app)
        .build();
    conf_css();
    window.set_child(Some(&mainBox));
    window.show();
}

pub fn load() {
    let app = gtk::Application::builder().application_id(id).build(); 
    app.connect_activate(on_active);
    app.run();
}

pub fn conf_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    let mut css_path = String::new();
    if let Some(css) = config_dir() {
        css_path = format!("{}/m3utohtml/css/app.css", css.display());
    } else {
        css_path = "./css/app.css".to_string();
    }
    let css_content: String = match parser::open_file(&Path::new(&css_path)) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./css/app.css")),
    };
    provider.load_from_data(&css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}
