use std::path::PathBuf;

use crate::{buttons::{self, gencover}, database, get_Arguments, logic, visual::set_TrackList};

pub fn openfile(file: PathBuf) {
    let args = get_Arguments();
    if args.vlcplaylist {
        let playlists = buttons::read_db_playlists(file, database::dbtype::Vlc_playlists);
        println!("{:?}", playlists);
    } else if let Some(vlc) = args.vlc {
        let (pl, tra) = buttons::read_db(file, database::dbtype::Vlc, &vlc);
        set_TrackList(pl.clone());
        if args.cover {
            let pl2 = gencover(pl, true);
            set_TrackList(pl2);
        }        
        logic::generate(&tra);
    } else {
        let (tra, pl) = buttons::read_file(file);
        set_TrackList(pl.clone());
        if args.cover {
            let pl2 = gencover(pl, true);
            set_TrackList(pl2);
        }        
        logic::generate(&tra);
    }
}
