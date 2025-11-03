use std::{path::{Path, PathBuf}, sync::Mutex};
use clap::{Arg, Parser, builder::Str};

use cli::openfile;
use logic::generate;
use visual::set_TrackList;

mod window;
mod visual;
mod logic;
mod parser;
mod error;
mod buttons;
mod database;
mod cli;

/// Gtk4 Application to convert playlists to html website
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// generate preview in html_path
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    preview: bool,

    /// use VLC database for cli mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    vlc: bool,

    /// generate covers for cli mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    cover: bool,

    /// cli mode file input
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// path to html folder
    #[arg(long, default_value = "./html")]
    html_path: PathBuf,

    /// path to css folder
    #[arg(long, default_value = "./css/main.css")]
    css_path: PathBuf,
}

static Arguments: Mutex<Vec<Args>> = Mutex::new(vec!());

pub fn set_Arguments(args: Args) {
    Arguments.lock().unwrap().clear();
    Arguments.lock().unwrap().push(args);
}

pub fn get_Arguments() -> Args {
    return Arguments.lock().unwrap().to_vec().first().unwrap().clone();
}

fn main() {
    let args = Args::parse();
    set_Arguments(args.clone());
    if args.preview {
        let prevList = vec!(logic::Track::example());
        set_TrackList(prevList);
        generate("{$playlistname}");
    } else if let Some(f) = args.input {
        openfile(f.clone(), args.vlc.clone(), args.cover.clone());
        return;
    } else {
        window::load();
    }
}
