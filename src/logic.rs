use std::path::PathBuf;

use gtk::prelude::*;
use lofty::{file::TaggedFileExt, tag::Accessor};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
   }

impl Track {
    pub fn new(path: PathBuf) -> Self {
        let mut art = "Unknown Artist".to_string();
        let mut tit = "Unknown Title".to_string();
        let mut alb = "Unknown Album".to_string();
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
        }
    }
    pub fn genBox(&self) -> gtk::Box {
        let trackBox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        let title = gtk::Label::new(Some(&self.title));
        let artist = gtk::Label::new(Some(&self.artist));
        let album = gtk::Label::new(Some(&self.album));
        let divide = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let divide2 = gtk::Box::new(gtk::Orientation::Vertical, 1);
        divide2.set_valign(gtk::Align::End);
        trackBox.append(&divide);
        trackBox.append(&divide2);
        divide.append(&album);
        divide.append(&title);
        divide2.append(&artist);
        return trackBox;
    }
}
