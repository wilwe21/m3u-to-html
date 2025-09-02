use std::{fs, path::{Path, PathBuf}};

use gtk::prelude::*;
use lofty::{file::TaggedFileExt, tag::Accessor};

use crate::parser;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub tracknum: String,
    pub maxtracknum: String,
   }

impl Track {
    pub fn new(path: PathBuf) -> Self {
        let mut art = "Unknown Artist".to_string();
        let mut tit = "Unknown Title".to_string();
        let mut alb = "Unknown Album".to_string();
        let mut trn = "".to_string();
        let mut mtrn = "".to_string();
        match lofty::read_from_path(path) {
        Ok(tagged_file) => {
            if let Some(tag) = tagged_file.primary_tag() {
                if let Some(a) = tag.artist() {
                    art = a.to_string();
                } 
                if let Some(t) = tag.title() {
                    tit = t.to_string();
                }
                if let Some(al) = tag.album() {
                    alb = al.to_string();
                }
                if let Some(tr) = tag.track() {
                    trn = tr.to_string();
                }
                if let Some(mtr) = tag.track_total() {
                    mtrn = mtr.to_string();
                }
            } else {
                println!("No tags found.");
            }
        },
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
        }
        }
        return Self {
            title: tit,
            artist: art,
            album: alb,
            tracknum: trn,
            maxtracknum: mtrn,
        }
    }
    pub fn genBox(&self) -> gtk::Box {
        let trackBox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        trackBox.add_css_class("track");
        let title = gtk::Label::new(Some(&self.title));
        trackBox.add_css_class("title");
        let artist = gtk::Label::new(Some(&self.artist));
        artist.add_css_class("artist");
        let album = gtk::Label::new(Some(&format!("{} Track number: {} / {}", self.album, self.tracknum, self.maxtracknum)));
        album.add_css_class("album");
        let divide = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(1)
            .hexpand(true)
            .build();
        let divide2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(1)
            .hexpand(true)
            .build();
        divide2.set_valign(gtk::Align::End);
        title.set_halign(gtk::Align::Start);
        album.set_halign(gtk::Align::Start);
        artist.set_halign(gtk::Align::End);
        trackBox.append(&divide);
        trackBox.append(&divide2);
        divide.append(&album);
        divide.append(&title);
        divide2.append(&artist);
        return trackBox;
    }

    pub fn getHTML(&self) -> String {
        let template: String = match parser::open_file(&Path::new("./html/track")) {
            Ok(file) => file,
            Err(_) => String::from(include_str!("./html/track")),
        };
        let mut output: String = String::new();
        for (index, line) in template.lines().enumerate() {
            match parser::parse_line(&line, &self) {
                Ok(line) => output.push_str(&line),
                Err(err) => {
                    eprintln!("Error in line {}: {}", index + 1, err);
                }
            }
        }
        return output;
    }
}
