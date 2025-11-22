use std::{fs::File, io::{self, Read}, path::Path};

use dirs::config_dir;

use crate::{artistslogic::{Album, Artist}, error::ConfigError, get_Arguments, logic::Track};

pub fn open_file(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_line(line: &str, track: Option<Track>, art: Option<Artist>, play: Option<String>, alb: Option<Album>) -> Result<String, ConfigError> {
    let mut buffer: String = String::new();
    let mut output: String = String::new();
    let mut inside_braces: bool = false;
    let mut escape_next: bool = false;

    for char in line.chars() {
        if escape_next {
            match char {
                '{' | '}' | '\\' => buffer.push(char),
                _ => {
                    buffer.push('\\');
                    buffer.push(char);
                }
            }
            escape_next = false;
            continue;
        }

        match char {
            '\\' => {
                if inside_braces {
                    escape_next = true;
                } else {
                    output.push('\\');
                }
            }
            '{' => {
                if inside_braces {
                    return Err(ConfigError::UnexpectedCurlyBrace);
                }
                inside_braces = true;
                buffer.clear();
            }
            '}' => {
                if !inside_braces {
                    return Err(ConfigError::UnexpectedCurlyBrace);
                }
                inside_braces = false;
                output.push_str(&parse_var(&buffer, track.clone(), art.clone(), play.clone(), alb.clone())?);
            }
            _ => {
                if inside_braces {
                    buffer.push(char);
                } else {
                    output.push(char);
                }
            }
        }
    }

    if inside_braces {
        return Err(ConfigError::UnexpectedCurlyBrace);
    }

    output.push('\n');
    Ok(output)
}

fn parse_var(var: &str, track: Option<Track>, art: Option<Artist>, playlist: Option<String>, alb: Option<Album>) -> Result<String, ConfigError> {
    match var {
        _ if var.starts_with('$') && track.is_some() => Ok(replace_var(&var[1..], &track.unwrap())?),
        _ if var.starts_with('@') && art.is_some() => Ok(replace_var_artist(&var[1..], &art.unwrap())?),
        _ if var.starts_with('!') && playlist.is_some() => Ok(replace_var_playlist(&var[1..], &playlist.unwrap())?),
        _ if var.starts_with('^') && alb.is_some() => Ok(replace_var_album(&var[1..], &alb.unwrap())?),
        _ if var.starts_with('%') => Ok(replace_var_css(&var[1..])?),
        _ => Err(ConfigError::UnknownVariable(String::from(var))),
    }
}

const BUILTIN_VARS: &[&str] = &["track", "album", "tracknumber", "artist", "cover"];

fn replace_var(key: &str, track: &Track) -> Result<String, ConfigError> {
    if !BUILTIN_VARS.contains(&key) {
        return Err(ConfigError::UnknownVariable(String::from(key)));
    }
    let mut numb = "".to_string();
    if track.tracknum != "".to_string() {
        numb.push_str(&format!("tracknumber: {}", track.tracknum));
        if track.maxtracknum != "".to_string() {
            numb.push_str(&format!(" / {}", track.maxtracknum));
        }
    }

    Ok(match key {
        "album" => track.album.clone(),
        "track" => track.title.clone(),
        "tracknumber" => numb.clone(),
        "artist" => track.artist.clone(),
        "cover" => track.cover.clone(),
        _ => unreachable!(),
    })
}

const BUILTIN_VARS_ARTIST: &[&str] = &["name", "cover", "description", "tags", "albums"];

fn replace_var_artist(key: &str, art: &Artist) -> Result<String, ConfigError> {
    if !BUILTIN_VARS_ARTIST.contains(&key) {
        return Err(ConfigError::UnknownVariable(String::from(key)));
    }

    Ok(match key {
        "name" => art.name.clone(),
        "cover" => art.cover.clone(),
        "description" => art.description.clone(),
        "tags" => art.tags.iter().map(|tag| format!("<p>{}</p>", tag)).collect::<Vec<_>>().join("\n").clone(),
        "albums" => for_albs(art.albums.clone()),
        _ => unreachable!(),
    })
}

fn for_albs(albums: Vec<Album>) -> String {
    let mut fin = String::new();
    for i in albums {
        let s = for_alb(i);
        fin.push_str(&s);
    }
    return fin;
}

fn for_alb(album: Album) -> String {
    let args = get_Arguments();
    let mut html_path = String::new();
    if let Some(html_p) = args.html_path {
        html_path = html_p.display().to_string();
    } else {
        if let Some(conf_dir) = config_dir() {
            html_path = format!("{}/m3utohtml/html", conf_dir.display());
        } else {
            html_path = "./html".to_string();
        }
    }
    let album_path = format!("{}/album", html_path);
    let template: String = match open_file(&Path::new(&album_path)) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./html/album")),
    };
    let mut output: String = String::new();
    for (index, line) in template.lines().enumerate() {
        match parse_line(&line, None, None, None, Some(album.clone())) {
            Ok(line) => output.push_str(&line),
            Err(err) => {
                eprintln!("Error in line {}: {}", index + 1, err);
            }
        }
    }
    return output;
}

const BUILTIN_VARS_ALBUM: &[&str] = &["name", "cover"];

fn replace_var_album(key: &str, alb: &Album) -> Result<String, ConfigError> {
    if !BUILTIN_VARS_ALBUM.contains(&key) {
        return Err(ConfigError::UnknownVariable(String::from(key)));
    }

    Ok(match key {
        "name" => alb.name.clone(),
        "cover" => alb.cover.clone(),
        _ => unreachable!(),
    })
}

fn replace_var_playlist(key: &str, playlist: &str) -> Result<String, ConfigError> {
    match key {
        "playlist" => Ok(playlist.to_string().clone()),
        _ => Err(ConfigError::UnknownVariable(String::from(key))),
    }
}

fn replace_var_css(key: &str) -> Result<String, ConfigError> {
    let args = get_Arguments();
    let mut css_loc = String::new();
    if let Some(css_path) = args.css_path {
        css_loc = format!("{}", css_path.display());
    } else {
        if let Some(conf_dir) = config_dir() {
            css_loc = format!("{}/m3utohtml/css/main.css", conf_dir.display());
        } else {
            css_loc = format!("./css/main.css");
        }
    }
    let css: String = match open_file(&Path::new(&css_loc)) {
        Ok(file) => file,
        Err(_) => String::from(include_str!("./css/main.css")),
    };

    match key {
        "css" => Ok(css.clone()),
        _ => Err(ConfigError::UnknownVariable(String::from(key))),
    }
}
