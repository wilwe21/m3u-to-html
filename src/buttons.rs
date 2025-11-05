use std::{clone, fs, path::PathBuf, str::FromStr, thread::sleep, time::Duration};

use gtk::{ResponseType, gio::LocalTask, glib::value, prelude::*};
use sqlx::Database;
use tokio::runtime::Runtime;

use crate::{database, logic::{self, Track, covers}, visual::{self, wind}};

pub fn importButtons(app: &gtk::Application, mBox: gtk::Box) -> gtk::Box {
    let horibox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    horibox.set_hexpand(true);
    let fb = fileButton(&app.clone(), mBox.clone());
    let vlcdbb = dbButton(&app.clone(), mBox.clone());
    horibox.append(&fb);
    horibox.append(&vlcdbb);
    return horibox;
}

pub fn fileButton(app: &gtk::Application, mBox: gtk::Box) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Choose File")
        .build();
    let filter = gtk::FileFilter::new();
    filter.add_mime_type("audio/x-mpegurl");
    let f = gtk::FileChooserDialog::builder()
        .title("Choose m3u file")
        .filter(&filter)
        .action(gtk::FileChooserAction::Open)
        .build();
    f.add_buttons(&[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)]);
    let appclone = app.clone();
    button.connect_clicked(move |_| {
        f.show();
        let val = appclone.clone();
        let mboxclone = mBox.clone();
        f.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path_buf) = file.path() {
                        println!("Selected file: {:?}", path_buf.display());
                        let (tra, pl) = read_file(path_buf);
                        visual::afterBox(&val.clone(),mboxclone.clone(), pl, tra);
                    } else {
                        println!("Could not get path from GFile.");
                    }
                }
            }
            dialog.hide();
        });
    });
    return button;
}

pub fn read_file(path: PathBuf) -> (String, Vec<Track>) {
    let mut filename = String::new();
    if let Some(name) = path.file_name() {
        filename = name.to_os_string().into_string().unwrap();
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
    return (filename, finList);
}

pub fn read_db(path: PathBuf, t: database::dbtype, name: &str) -> (Vec<Track>, String) {
    let request = database::dbRequest(path.display().to_string(), t, name);
    let rt = Runtime::new().unwrap();

    return rt.block_on(request).unwrap();
}

pub fn read_db_playlists(path: PathBuf, t: database::dbtype) -> Vec<String> {
    let request = database::dbRequestPlaylists(path.display().to_string(), t);
    let rt = Runtime::new().unwrap();

    return rt.block_on(request).unwrap();
}

pub fn dbButton(app: &gtk::Application, mBox: gtk::Box) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Choose VLC DB")
        .build();
    let f = gtk::FileChooserDialog::builder()
        .title("Choose m3u file")
        .action(gtk::FileChooserAction::Open)
        .build();
    f.add_buttons(&[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)]);
    let appclone = app.clone();
    button.connect_clicked(move |_| {
        f.show();
        let mboxclone = mBox.clone();
        let val = appclone.clone();
        f.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path_buf) = file.path() {
                        let (pl, tra) = read_db(path_buf, database::dbtype::Vlc, "favorite");
                        visual::afterBox(&val.clone(),mboxclone.clone(), pl, tra);
                    } else {
                        println!("Could not get path from GFile.");
                    }
                }
            }
            dialog.hide();
        });
    });
    return button;
}
pub fn getCoversButton(app: &gtk::Application) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Get Covers")
        .build();
    let appclone = app.clone();

    let (progres, window) = visual::coverLoading(&appclone.clone());
    progres.set_fraction(0.0);
    let progclon = progres.clone();
    let winclon = window.clone();

    button.connect_clicked(move |_| {
        let (sender, receiver) = async_channel::unbounded::<String>();
        winclon.present();
        winclon.show();
        winclon.set_visible(true);
        let endsig = "exit".to_string();
        let value = progclon.clone();
        let value2 = winclon.clone();
        let value3 = endsig.clone();
        gtk::glib::spawn_future_local(async move {
            let progclonclon = value.clone();
            let winclonclon = value2.clone();
            let endsigclon = value3.clone();
            while let Ok(stat) = receiver.recv().await {
                match stat {
                    val if val == endsigclon.clone() => winclonclon.hide(),
                    _ => progclonclon.set_fraction(stat.parse().unwrap())
                }
            }
        });
        gtk::gio::spawn_blocking(move || {
            let tracks = visual::get_TrackList();
            let size =tracks.clone().len();
            let mut new: Vec<Track> = vec!();
            for (n,mut t) in &mut tracks.clone().into_iter().enumerate() {
                let nt = logic::covers(&mut t);
                new.push(nt.clone());
                sender.send_blocking(((n+1) as f64/size as f64).to_string());
            }
            visual::set_TrackList(new.clone());
            sender.send_blocking(endsig.clone());
            println!("[log] generated Covers");
        });
    });
    return button;
}

pub fn gencover(tracks: Vec<Track>, print: bool) -> Vec<Track> {
    let mut new = vec!();
    let size = tracks.len();
    for (n, mut t) in &mut tracks.clone().into_iter().enumerate() {
        let nt = logic::covers(&mut t);
        new.push(nt.clone());
        if print {
            if n > 0 {
                println!("\x1B[1A{}/{}", n+1, size);
            } else {
                println!("{}/{}", n+1, size);
            }
        }
    }
    return new;
}
