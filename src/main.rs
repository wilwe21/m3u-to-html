#![windows_subsystem = "windows"]
mod window;
mod visual;
mod logic;
mod parser;
mod error;
mod buttons;
mod database;

fn main() {
    window::load();
}
