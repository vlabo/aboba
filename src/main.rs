// extern crate ffmpeg_next as ffmpeg;
mod audio;
mod file_manager;
mod ui;
mod util;

use gtk::prelude::*;
use ui::*;

fn build_ui(application: &gtk::Application) {
    let ui = Ui::build_ui(application);
    ui.setup_open_button();
    ui.run();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.vlabo.aboba"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
