#![windows_subsystem = "windows"]
mod window;
mod visual;
mod logic;
mod parser;
mod error;
mod buttons;

fn main() {
    window::load();
}
