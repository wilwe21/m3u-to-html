use std::{path::{PathBuf}, sync::Mutex};
use clap::{Parser};

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
mod cache;
mod artistslogic;

/// Gtk4 Application to convert playlists to html website
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// generate preview in html_path
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    preview: bool,

    /// use VLC database for cli mode
    #[arg(short, long)]
    vlc: Option<String>,

    /// show VLC playlists names cli mode
    #[arg(short = 'P', long, action = clap::ArgAction::SetTrue)]
    vlcplaylist: bool,

    /// generate covers for cli mode
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    cover: bool,

    /// cli mode file input
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// path to html folder default config_dir/html
    #[arg(long)]
    html_path: Option<PathBuf>,

    /// path to css file default config_dir/css/main.css
    #[arg(long)]
    css_path: Option<PathBuf>,

    /// output file input default ./{playlistname}_playlist.html
    #[arg(short, long)]
    output: Option<PathBuf>,
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
        openfile(f.clone());
        return;
    } else {
        window::load();
    }
}
