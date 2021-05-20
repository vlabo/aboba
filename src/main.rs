extern crate ffmpeg_next as ffmpeg;
extern crate gstreamer as gst;

extern crate rodio;

#[macro_use]
extern crate lazy_static;

mod audio;
mod filemanager;
mod ui;
mod util;

use gtk::prelude::*;
use ui::*;

lazy_static! {
    static ref PLAYER: audio::Player = audio::Player::setup().unwrap();
}

fn setup(file: &str) -> audio::Control {
    PLAYER.set_file(&file.to_owned());
    return PLAYER.new_control();
}

fn build_ui(application: &gtk::Application) {
    let ui = Ui::build_ui(application);
    ui.setup_open_button(&setup);
    ui.run();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.vlabo.aboba"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
