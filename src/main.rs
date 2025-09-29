mod config;
mod gui;
mod rsync;
mod worker;

use crate::gui::run_gui;

use dirs; // cargo add dirs
use toml; // cargo add toml

fn main() {
    run_gui();
}
