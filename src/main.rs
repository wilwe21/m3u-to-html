#![windows_subsystem = "windows"]
mod window;
mod visual;
mod logic;
mod parser;
mod error;

fn main() {
    window::load();
}
