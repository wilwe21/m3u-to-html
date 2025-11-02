use std::{fs::File, io::{self, Read}, path::Path};

use crate::{error::ConfigError, get_Arguments, logic::Track};

pub fn open_file(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_line(line: &str, track: &Track) -> Result<String, ConfigError> {
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
                output.push_str(&parse_var(&buffer, track)?);
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

fn parse_var(var: &str, track: &Track) -> Result<String, ConfigError> {
    match var {
        _ if var.starts_with('$') => Ok(replace_var(&var[1..], track)?),
        _ => Err(ConfigError::UnknownVariable(String::from(var))),
    }
}
pub fn parse_line_playlist(line: &str, playlist: &str) -> Result<String, ConfigError> {
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
                output.push_str(&parse_var_playlist(&buffer, playlist)?);
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

fn parse_var_playlist(var: &str, playlist: &str) -> Result<String, ConfigError> {
    match var {
        _ if var.starts_with('$') => Ok(replace_var_playlist(&var[1..], playlist)?),
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

fn replace_var_playlist(key: &str, playlist: &str) -> Result<String, ConfigError> {
    if key == "playlist" || key == "css" {
        let args = get_Arguments();
        let css_loc = format!("{}", args.css_path.display());
        let css: String = match open_file(&Path::new(&css_loc)) {
            Ok(file) => file,
            Err(_) => String::from(include_str!("./css/main.css")),
        };

        Ok(match key {
            "playlist" => playlist.to_string().clone(),
            "css" => css.clone(),
            _ => unreachable!(),
        })
    } else {
        return Err(ConfigError::UnknownVariable(String::from(key)));
    }
}

