mod audio;
mod file_manager;
mod ui;

use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let ui = ui::Ui::build_ui(application);
    ui.run();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.vlabo.aboba"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
