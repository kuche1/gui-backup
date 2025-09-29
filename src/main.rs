mod config;
mod gui;
mod rsync;
mod worker;

use crate::gui::run_gui;

fn main() {
    run_gui();
}
