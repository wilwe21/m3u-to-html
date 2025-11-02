use sqlx::{Pool, Sqlite, prelude::FromRow, sqlite::SqliteConnectOptions};

use crate::logic::Track;

pub enum dbtype {
    Vlc,
    Spotify
}

#[derive(FromRow, Debug, Clone)]
struct data {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    number: u8
}

pub async fn dbRequest(db: String, t: dbtype) -> Option<(Vec<Track>, String)> {
    let opt = SqliteConnectOptions::new().filename(&db)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete);
    let connection = sqlx::sqlite::SqlitePool::connect_with(opt).await.unwrap();
    match t {
        dbtype::Vlc => Some(vlc(connection.clone()).await),
        dbtype::Spotify => {println!("spotify"); return None}
    }
}

async fn vlc(connection: Pool<Sqlite>) -> (Vec<Track>, String) {
    let med: Vec<data> = sqlx::query_as("
        SELECT 
        media.title, 
        artist.name AS artist, 
        album.title AS album, 
        media.track_number AS number
        FROM media 
        INNER JOIN artist ON media.artist_id=artist.id_artist 
        INNER JOIN album ON media.album_id=id_album 
        WHERE media.is_favorite = 1;"
    ).fetch_all(&connection).await.unwrap();
    let mut vec = vec!();
    for d in med {
        vec.push(d.toTrack());
    }
    return (vec.clone(), "fav".to_string());
}

impl data {
    fn toTrack(self) -> Track {
        let mut tit = "Unknown Title".to_string();
        let mut art = "Unknown Artist".to_string();
        let mut alb = "Single".to_string();
        let mut trn = "".to_string();
        let mut mtrn = "".to_string();
        let cov = "https://lastfm.freetls.fastly.net/i/u/770x0/0248ee38f8d45f32fe6fad5d43e64f47.jpg#0248ee38f8d45f32fe6fad5d43e64f47".to_string();
        if let Some(a) = self.title {
            tit = a;
        }
        if let Some(a) = self.artist {
            art = a;
        }
        if let Some(a) = self.album {
            alb = a;
        }
        if self.number > 0 {
            trn = self.number.to_string();
        }
        return Track {
            title: tit,
            artist: art,
            album: alb,
            tracknum: trn,
            maxtracknum: mtrn,
            cover: cov,
        }
    }
}
