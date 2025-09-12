use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}};

use gtk::{glib::property::PropertyGet, prelude::*, ResponseType};

use crate::{logic, parser};

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
        for el in &finList {
            end.push_str(&el.getHTML());
        }
        end.push_str(&tail);
        gen_output(&end, &filename);
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
