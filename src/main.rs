mod audio;
mod file_manager;
mod ui;

use gtk::gdk;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {

    let provider = gtk::CssProvider::new();
    let style = include_bytes!("ui/style.css");
    provider.load_from_data(style).expect("Failed to load CSS");
    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let ui = ui::Ui::build_ui(application);
    ui.run();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.vlabo.aboba"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
