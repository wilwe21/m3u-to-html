use std::path::PathBuf;

use crate::{buttons::{self, gencover}, logic, visual::set_TrackList};

pub fn openfile(file: PathBuf, vlc: bool, cover: bool) {
    if !vlc {
        let (tra, pl) = buttons::read_file(file);
        set_TrackList(pl.clone());
        if cover {
            let pl2 = gencover(pl, true);
            set_TrackList(pl2);
        }        
        logic::generate(&tra);
    } else {
        let (pl, tra) = buttons::read_db(file);
        set_TrackList(pl.clone());
        if cover {
            let pl2 = gencover(pl, true);
            set_TrackList(pl2);
        }        
        logic::generate(&tra);
    }
}
