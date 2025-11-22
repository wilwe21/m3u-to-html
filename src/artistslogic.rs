use std::path::Path;

use dirs::config_dir;

use crate::{get_Arguments, parser};


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Artist {
    pub name: String,
    pub cover: String,
    pub description: String,
    pub tags: Vec<String>,
    pub albums: Vec<Album>
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Album {
    pub name: String,
    pub cover: String
}

impl Artist {
    pub fn new(name: String, cover: String, description: String, tags: Vec<String>, albums: Vec<Album>) -> Self {
        return Self {
            name,
            cover,
            description,
            tags,
            albums
        };
    }

    pub fn example() -> Self {
        let name = "Example".to_string();
        let cover = "https://lastfm.freetls.fastly.net/i/u/770x0/0248ee38f8d45f32fe6fad5d43e64f47.jpg#0248ee38f8d45f32fe6fad5d43e64f47".to_string();
        let description = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();
        let tags = vec!("Pop".to_string(), "Rock".to_string(), "Electro".to_string());
        let albums: Vec<Album> = vec!(Album::example(), Album::example(), Album::example());
        return Self {
            name,
            cover,
            description,
            tags,
            albums
        };
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn getHTML(self) -> String {
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
        let art_loc = format!("{}/artist", html_path);
        let template = match parser::open_file(&Path::new(&art_loc)) {
            Ok(file) => file,
            Err(_) => String::from(include_str!("./html/artist")),
        };
        let mut output: String = String::new();
        for (index, line) in template.lines().enumerate() {
            match parser::parse_line(&line, None, Some(self.clone()), None) {
                Ok(line) => output.push_str(&line),
                Err(err) => {
                    eprintln!("Error in line {}: {}", index + 1, err);
                }
            }
        }
        return output;
    }
}

impl Album {
    pub fn new(name: String, cover: String) -> Self {
        return Self {
            name,
            cover
        };
    }
    pub fn example() -> Self {
        let name = "Example".to_string();
        let cover = "https://lastfm.freetls.fastly.net/i/u/770x0/0248ee38f8d45f32fe6fad5d43e64f47.jpg#0248ee38f8d45f32fe6fad5d43e64f47".to_string();
        return Self {
            name,
            cover
        };
    }
}
