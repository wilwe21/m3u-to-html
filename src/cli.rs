use std::path::PathBuf;

use crate::{buttons, logic, visual::set_TrackList};

pub fn openfile(file: PathBuf, vlc: bool) {
    if !vlc {
        let (tra, pl) = buttons::read_file(file);
        set_TrackList(pl);
        logic::generate(&tra);
    } else {
        let (pl, tra) = buttons::read_db(file);
        set_TrackList(pl);
        logic::generate(&tra);
    }
}
