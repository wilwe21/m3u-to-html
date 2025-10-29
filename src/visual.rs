use std::{fs::{self, File}, io::{Read, Write}, iter::Enumerate, path::{Path, PathBuf}, time::Duration};

use gtk::{ResponseType, gio::ffi::GApplication, glib::property::PropertyGet, prelude::*};
use sqlx::prelude::*;
use tokio::runtime::Runtime;
use std::sync::Mutex;

use crate::{logic, main, parser, window, buttons};

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
    let button = buttons::fileButton(&app.clone(), mainBox.clone());
    let dbbutton = buttons::dbButton(&app.clone(),mainBox.clone());
    mainBox.append(&button);
    mainBox.append(&dbbutton);
    return mainBox;
}

pub fn afterBox(app: &gtk::Application, mbox: gtk::Box, file: gtk::gio::File, path: PathBuf) {
    while let Some(child) = mbox.first_child() {
        mbox.remove(&child);
    }
    let button = buttons::fileButton(&app.clone(),mbox.clone());
    mbox.append(&button);
    let mut filename = String::new();
    if let Some(name) = path.file_name() {
        filename = name.to_os_string().into_string().unwrap();
        let fileName = gtk::Label::new(Some(&filename));
        mbox.append(&fileName);
    }
    let f = fs::read_to_string(&path).expect("wrong file");
    let list = f.split("\n");
    let mut finList = vec!();
    for s in list {
        if s != "" {
            let h = logic::Track::new(s.to_string().into());
            if let Some(hrt) = h{
                finList.push(hrt);
            }
        }
    }
    set_TrackList(finList.clone());
    let scrollBox = gtk::ListBox::new();
    for t in &finList {
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
            match parser::parse_line_playlist(line, &filename) {
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
        gen_output(&end, &filename);
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
    let mbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();    
    let lab = gtk::Label::new(Some("Loaging Covers"));
    mbox.append(&lab);
    let bind = app.windows();
    let parrent = bind.get(0).unwrap();
    let window = gtk::Window::builder()
        .title("Loading Covers")
        .modal(true)
        .application(app)
        .destroy_with_parent(true)
        .visible(false)
        .transient_for(parrent)
        .build();
    window.set_child(Some(&mbox));
    let progress = gtk::ProgressBar::new();
    progress.set_hexpand(true);
    progress.set_vexpand(true);
    mbox.append(&progress);
    return (progress, window);
}
