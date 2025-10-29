use std::{clone, thread::sleep, time::Duration};

use gtk::{ResponseType, gio::LocalTask, glib::value, prelude::*};
use tokio::runtime::Runtime;

use crate::{logic::{self, Track, covers}, visual::{self, wind}};

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
                        visual::afterBox(&val.clone(),mboxclone.clone(), file, path_buf);
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

pub fn dbButton(app: &gtk::Application, mBox: gtk::Box) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Choose VLC DB")
        .build();
    let f = gtk::FileChooserDialog::builder()
        .title("Choose m3u file")
        .action(gtk::FileChooserAction::Open)
        .build();
    f.add_buttons(&[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)]);
    button.connect_clicked(move |_| {
        f.show();
        let mboxclone = mBox.clone();
        f.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path_buf) = file.path() {
                        let pa = format!("sqlite:{}", path_buf.display());
                        println!("{}", &pa);
                        let sql = sqlx::sqlite::SqlitePoolOptions::new()
                            .max_connections(5)
                            .min_connections(1)
                            .acquire_timeout(Duration::from_secs(3))
                            .connect(&pa);
                        let rt = Runtime::new().unwrap();
                        let fut_sql = rt.block_on(sql);
                        if let Ok(s) = fut_sql {
                            println!("{:?}", s);
                        }
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

