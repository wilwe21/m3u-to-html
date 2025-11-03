use std::{fs::File, io::Write, path::{Path, PathBuf}};

use gtk::prelude::*;
use lofty::{file::TaggedFileExt, tag::Accessor};

use quick_xml::Reader;
use quick_xml::events::{Event};
use tokio::runtime::Runtime;

use crate::{get_Arguments, parser, visual::get_TrackList};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub tracknum: String,
    pub maxtracknum: String,
    pub cover: String,
}

impl Track {
    pub fn new(path: PathBuf) -> Option<Self> {
        let mut art = "Unknown Artist".to_string();
        let mut tit = "Unknown Title".to_string();
        let mut alb = "Single".to_string();
        let mut trn = "".to_string();
        let mut mtrn = "".to_string();
        let cov = "https://lastfm.freetls.fastly.net/i/u/770x0/0248ee38f8d45f32fe6fad5d43e64f47.jpg#0248ee38f8d45f32fe6fad5d43e64f47".to_string();
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
        if art == "Unknown Artist".to_string() {
            return None;
        }
        if tit == "Unknown Title".to_string() {
            return None;
        }

        return Some(Self {
            title: tit,
            artist: art,
            album: alb,
            tracknum: trn,
            maxtracknum: mtrn,
            cover: cov,
        })
    }

    pub fn example() -> Self {
        let cov = "https://lastfm.freetls.fastly.net/i/u/770x0/0248ee38f8d45f32fe6fad5d43e64f47.jpg#0248ee38f8d45f32fe6fad5d43e64f47".to_string();
        return Self {
            title: "{$title}".to_string(),
            artist: "{$artist}".to_string(),
            album: "{$album}".to_string(),
            tracknum: "{$tracknumber}".to_string(),
            maxtracknum: "".to_string(),
            cover: cov,
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
    pub fn changeCover(&mut self, cover: &str) {
        self.cover = cover.to_string();
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

pub fn covers(track: &mut Track) -> &mut Track {
    let mut cover_search_link = "http://ws.audioscrobbler.com/2.0/?method=album.getinfo&api_key=".to_string();
    let key = match parser::open_file(&Path::new("./html/token.txt")) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./html/token.txt"))
    }.replace("\n", "");
    if track.album == "Single".to_string() {
        cover_search_link.push_str(&format!("{}&artist={}&album={}", key,track.artist.replace("&", "%26"),track.title.replace("&", "%26")));
    } else {
        cover_search_link.push_str(&format!("{}&artist={}&album={}", key,track.artist.replace("&", "%26"),track.album.replace("&", "%26")));
    }
    let request = req(&cover_search_link);
    let rt = Runtime::new().unwrap();

    let end_cov = rt.block_on(request);
    if end_cov != "" {
        track.changeCover(&end_cov);
    }
    if track.cover != end_cov {
        println!("[log] Default cover");
    }
    return track;
}

pub async fn req(url: &str) -> String {
    let request = reqwest::get(url).await.expect("Request Timeout").text().await.expect("Wrong Request");

    let mut album_art_url: Option<String> = None;

    let mut reader = Reader::from_str(&request);

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name() == quick_xml::name::QName(b"image") => {
                let mut size_attr = None;
                for a in e.attributes() {
                    let attr = a.unwrap();
                    if attr.key == quick_xml::name::QName(b"size") {
                        size_attr = Some(String::from_utf8(attr.value.into_owned()).unwrap());
                    }
                }

                // Get the text content of the <image> tag
                let text = reader.read_text(e.name()).unwrap();

                if let Some(size) = size_attr {
                    if size == "large" {
                        album_art_url = Some(text.to_string());
                    }
                }
            },
            Ok(Event::Eof) => break, // Exit loop when EOF is reached
            _ => (), // Ignore other events
        }
    }

    if let Some(url) = album_art_url {
        return url.to_string();
    } else {
        return "".to_string();
    }
}

pub fn generate(playlistname: &str) {
    let args = get_Arguments();
    let top_loc = format!("{}/playlist", args.html_path.display());
    let top_template: String = match parser::open_file(&Path::new(&top_loc)) {
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
    let head_loc = format!("{}/header", args.html_path.display());
    let header_template: String = match parser::open_file(&Path::new(&head_loc)) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./html/header")),
    };
    let mut header = String::new();
    for (index, line) in header_template.lines().enumerate() {
        match parser::parse_line_playlist(line, &playlistname) {
            Ok(line) => header.push_str(&line),
            Err(err) => {
                eprint!("Error in line {}: {}", index+1, err);
            }
        }
    }
    end.push_str(&header);
    let tail_loc = format!("{}/tai;", args.html_path.display());
    let tail: String = match parser::open_file(&Path::new(&tail_loc)) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./html/tail")),
    };
    end.push_str(&top);
    for el in &get_TrackList() {
        end.push_str(&el.getHTML());
    }
    end.push_str(&tail);
    gen_output(&end, &playlistname);
    println!("[log] created");
}

fn gen_output(end: &str, filename: &str) {
    let mut output = File::create(format!("{}_playlist.html", filename));
    match output {
        Ok(mut o) => {o.write(end.as_bytes());},
        _ => {},
    }
}
