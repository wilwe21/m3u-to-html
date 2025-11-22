use std::{fs::File, io::Write, path::{Path, PathBuf}};

use gtk::{prelude::*};
use std::sync::Mutex;

use crate::{artistslogic, buttons::{self, read_db}, database::{dbtype, plays}, logic::{self, Track}, parser};

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

static ArtistList: Mutex<Vec<artistslogic::Artist>> = Mutex::new(vec!());

pub fn set_ArtistList(arts: Vec<artistslogic::Artist>) {
    ArtistList.lock().unwrap().clear();
    for t in arts.clone() {
        ArtistList.lock().unwrap().push(t);
    }
}

pub fn get_ArtistList() -> Vec<artistslogic::Artist> {
    return ArtistList.lock().unwrap().to_vec();
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
    let getart = buttons::getArtistsData(&app.clone());
    mbox.append(&getart);
    let create = gtk::Button::builder()
        .label("Create")
        .build();
    create.connect_clicked(move |_| {
        logic::generate(&playlistname.clone());
    });
    mbox.append(&create);
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

pub fn playlistChooseBox(app: &gtk::Application, mbox: gtk::Box, buf: PathBuf, playlists: Vec<String>) {
    while let Some(child) = mbox.first_child() {
        mbox.remove(&child);
    }
    let button = buttons::importButtons(&app.clone(),mbox.clone());
    mbox.append(&button);
    let scrollBox = gtk::ListBox::new();
    for i in playlists {
        let b = playButton(&app.clone(), mbox.clone(), buf.clone(), i.clone());
        scrollBox.append(&b);
    }
    let scroll = gtk::ScrolledWindow::builder()
        .child(&scrollBox)
        .vexpand(true)
        .build();
    mbox.append(&scroll);
}

fn playButton(app: &gtk::Application, mbox: gtk::Box, buf: PathBuf, playname: String) -> gtk::Button {
    let bu = gtk::Button::new();
    bu.set_label(&playname);
    let mbc = mbox.clone();
    let apc = app.clone();
    let plnc = playname.clone();
    let pbc = buf.clone();
    bu.connect_clicked(move |_| {
        let (pl, tra) = read_db(pbc.clone(), dbtype::Vlc, &plnc.clone());
        afterBox(&apc.clone(),mbc.clone(), pl, tra);
    });
    return bu;
}
