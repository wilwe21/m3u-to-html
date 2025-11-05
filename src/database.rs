use sqlx::{Pool, Sqlite, prelude::FromRow, sqlite::SqliteConnectOptions};

use crate::logic::{self, Track};

pub enum dbtype {
    Vlc,
    Vlc_playlists,
    Spotify
}

#[derive(FromRow, Debug, Clone)]
struct data {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    number: u8
}

#[derive(FromRow, Debug, Clone)]
struct plays {
    name: String
}

static playlists: &str ="
    SELECT
    name from playlist;
    ";

static fav: &str = "
    SELECT 
    media.title, 
    artist.name AS artist, 
    album.title AS album, 
    media.track_number AS number
    FROM media 
    INNER JOIN artist ON media.artist_id=artist.id_artist 
    INNER JOIN album ON media.album_id=id_album 
    WHERE media.is_favorite = 1;
    ";

static other: &str = "
    SELECT 
    media.title, 
    artist.name AS artist, 
    album.title AS album, 
    media.track_number AS number
    FROM media 
    INNER JOIN artist ON media.artist_id=artist.id_artist 
    INNER JOIN album ON media.album_id=id_album 
		INNER JOIN playlistmediarelation AS pmr ON media.id_media=pmr.media_id
		INNER JOIN playlist ON pmr.playlist_id=playlist.id_playlist
    WHERE playlist.name = \"DUPA\";
    ";

pub async fn dbRequest(db: String, t: dbtype, name: &str) -> Option<(Vec<Track>, String)> {
    let opt = SqliteConnectOptions::new().filename(&db)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete);
    let connection = sqlx::sqlite::SqlitePool::connect_with(opt).await.unwrap();
    match t {
        dbtype::Vlc => Some(vlc(connection.clone(), name.clone()).await),
        dbtype::Spotify => {println!("spotify"); return None},
        _ => None
    }
}
pub async fn dbRequestPlaylists(db: String, t: dbtype) -> Option<(Vec<String>)> {
    let opt = SqliteConnectOptions::new().filename(&db)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete);
    let connection = sqlx::sqlite::SqlitePool::connect_with(opt).await.unwrap();
    match t {
        dbtype::Vlc_playlists => Some(vlc_plays(connection.clone()).await),
        _ => None
    }
}

async fn vlc(connection: Pool<Sqlite>, name: &str) -> (Vec<Track>, String) {
    let med: Vec<data>;
    println!("{}", name.clone());
    if name == "favorite" {
        med = sqlx::query_as(&fav).fetch_all(&connection).await.unwrap();
    } else {
        med = sqlx::query_as(&other.replace("DUPA", name)).fetch_all(&connection).await.unwrap();
    }
    let mut vec = vec!();
    for d in med {
        vec.push(d.toTrack());
    }
    return (vec.clone(), name.to_string());
}

async fn vlc_plays(connection: Pool<Sqlite>) -> Vec<String> {
    let play: Vec<plays> = sqlx::query_as(&playlists).fetch_all(&connection).await.unwrap();
    let mut pl2: Vec<String> = play.iter().map(|a| a.name.to_string()).collect();
    pl2.push("favorite".to_string());
    return pl2;
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
