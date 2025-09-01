use std::{fs::{self, File}, io::Read, path::PathBuf};

use gtk::{glib::property::PropertyGet, prelude::*, ResponseType};

use crate::logic;

pub fn wind() -> gtk::Box {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let button = fileButton(mainBox.clone());
    mainBox.append(&button);
    return mainBox;
}

fn fileButton(mBox: gtk::Box) -> gtk::Button {
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
    button.connect_clicked(move |_| {
        f.show();
        let mboxclone = mBox.clone();
        f.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path_buf) = file.path() {
                        println!("Selected file: {:?}", path_buf.display());
                        afterBox(mboxclone.clone(), file, path_buf);
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

fn afterBox(mbox: gtk::Box, file: gtk::gio::File, path: PathBuf) {
    while let Some(child) = mbox.first_child() {
        mbox.remove(&child);
    }
    let button = fileButton(mbox.clone());
    mbox.append(&button);
    if let Some(name) = path.file_name() {
        let fileName = gtk::Label::new(name.to_str());
        mbox.append(&fileName);
    }
    let f = fs::read_to_string(&path).expect("wrong file");
    let list = f.split("\n");
    let mut finList = vec!();
    for s in list {
        if (s != "") {
            let h = logic::Track::new(s.to_string().into());
            finList.push(h);
        }
    }
    for t in finList {
        let tr_box = t.genBox();
        mbox.append(&tr_box);
    }
}
