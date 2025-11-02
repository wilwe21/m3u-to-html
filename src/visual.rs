use std::{fs::{self, File}, io::{Read, Write}, iter::Enumerate, path::{Path, PathBuf}, time::Duration};

use gtk::{ResponseType, gio::ffi::GApplication, glib::property::PropertyGet, prelude::*};
use sqlx::prelude::*;
use tokio::runtime::Runtime;
use std::sync::Mutex;

use crate::{buttons, logic::{self, Track}, main, parser, window};

static TrackList: Mutex<Vec<logic::Track>> = Mutex::new(vec!());

pub fn set_TrackList(tracks: Vec<logic::Track>) {
    TrackList.lock().unwrap().clear();
    for t in tracks.clone() {
        TrackList.lock().unwrap().push(t);
    }
}

pub fn get_TrackList() -> Vec<logic::Track> {
    return TrackList.lock().unwrap().to_vec();
}


pub fn wind(app: &gtk::Application) -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let buttons = buttons::importButtons(&app.clone(), mainBox.clone());
    mainBox.append(&buttons);
    return mainBox;
}

pub fn afterBox(app: &gtk::Application, mbox: gtk::Box, tracks: Vec<Track>, playlistname: String) {
    while let Some(child) = mbox.first_child() {
        mbox.remove(&child);
    }
    let button = buttons::importButtons(&app.clone(),mbox.clone());
    mbox.append(&button);
    let play = gtk::Label::new(Some(&playlistname));
    mbox.append(&play);
    set_TrackList(tracks.clone());
    let scrollBox = gtk::ListBox::new();
    for t in &tracks {
        let tr_box = t.genBox();
        scrollBox.append(&tr_box);
    }
    let scroll = gtk::ScrolledWindow::builder()
        .child(&scrollBox)
        .vexpand(true)
        .build();
    mbox.append(&scroll);
    let getcov = buttons::getCoversButton(&app.clone());
    mbox.append(&getcov);
    let create = gtk::Button::builder()
        .label("Create")
        .build();
    create.connect_clicked(move |_| {
        let top_template: String = match parser::open_file(&Path::new("./html/playlist")) {
            Ok(file) => file,
            Err(_) => String::from(include_str!("./html/playlist")),
        };
        let mut top = String::new();
        for (index, line) in top_template.lines().enumerate() {
            match parser::parse_line_playlist(line, &playlistname) {
                Ok(line) => top.push_str(&line),
                Err(err) => {
                    eprint!("Error in line {}: {}", index+1, err);
                }
            }
        }
        let mut end = String::new();
        let header = include_str!("./html/header");
        end.push_str(&header);
        let tail = include_str!("./html/tail");
        end.push_str(&top);
        for el in &get_TrackList() {
            end.push_str(&el.getHTML());
        }
        end.push_str(&tail);
        gen_output(&end, &playlistname);
        println!("[log] created");
    });
    mbox.append(&create);
}

fn gen_output(end: &str, filename: &str) {
    let mut output = File::create(format!("{}_playlist.html", filename));
    match output {
        Ok(mut o) => {o.write(end.as_bytes());},
        _ => {},
    }
}

pub fn coverLoading(app: &gtk::Application) -> (gtk::ProgressBar, gtk::Window) {
    let mbox = gtk::CenterBox::new();
    mbox.set_orientation(gtk::Orientation::Vertical);
    let lab = gtk::Label::new(Some("Loaging Covers"));
    mbox.set_start_widget(Some(&lab));
    let bind = app.windows();
    let parrent = bind.get(0).unwrap();
    let window = gtk::Window::builder()
        .width_request(380)
        .height_request(150)
        .title("Loading Covers")
        .modal(true)
        .application(app)
        .destroy_with_parent(true)
        .resizable(false)
        .visible(false)
        .transient_for(parrent)
        .build();
    window.set_child(Some(&mbox));
    let progress = gtk::ProgressBar::new();
    progress.set_hexpand(true);
    progress.set_vexpand(true);
    progress.set_margin_top(90);
    mbox.set_center_widget(Some(&progress));
    return (progress, window);
}
