use gtk::prelude::*;
// use gio::prelude::*;

mod player;
mod chapters;

use player::*;
use chapters::*;
use gtk::glib;

pub struct Ui {
    window: gtk::Window,
    player: Player,
    chapters: Chapters,
    open_button: gtk::Button,
}

impl Ui {

    pub fn build_ui(application: &gtk::Application) -> Self {

        let builder = gtk::WindowBuilder::new();
        let window = builder.application(application)
        .width_request(300).height_request(500).build();

        window.set_border_width(0);
        window.set_position(gtk::WindowPosition::Center);

        // Header
        let bar = gtk::HeaderBar::new();
        bar.set_show_close_button(true);
        let open_button = gtk::Button::with_label("Open");
        bar.add(&open_button);
        window.set_titlebar(Some(&bar));

        // Player
        let player = Player::new();
        let player_container = player.get_container();

        // Chapters
        let chapters = Chapters::new();
        let chapters_container = chapters.get_container();

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

        stack.add(player.get_container());
        stack.add(chapters.get_container());

        chapters.set_back_fn(glib::clone!(@weak stack, @weak player_container => move |_| {
            stack.set_visible_child(&player_container);
        }));

        player.set_open_chapters_fn(glib::clone!(@weak stack, @weak chapters_container => move |_| {
            stack.set_visible_child(&chapters_container);
        }));

        window.add(&stack);

        return Self{window, player, chapters, open_button};
    }

    pub fn setup_open_button(&self, get_control: &'static dyn Fn(&str) -> super::audio::Control) {
        let open_button = &self.open_button;
        let window = &self.window;
        let chapters = &self.chapters;
        let player = &self.player;
        open_button.connect_clicked(glib::clone!(@weak window, @weak chapters, @weak player => move |_| {

            let file_chooser = gtk::FileChooserDialog::new(
                Some("Open File"),
                Some(&window),
                gtk::FileChooserAction::Open,
            );
            file_chooser.add_buttons(&[
                ("Open", gtk::ResponseType::Ok),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);
            file_chooser.connect_response(move |file_chooser, response| {
                if response == gtk::ResponseType::Ok {
                    let file = file_chooser.filename().expect("Couldn't get filename");
                    if let Some(file_str) = file.as_path().to_str() {
                        let book = super::filemanager::get_book(file_str);
                        let control = get_control(file_str);
                        chapters.set_chapters(&book.chapters, control.clone());
                        player.initialize_book(&book.title, &book.chapters, control);
                    }
                }
                file_chooser.close();
            });

            file_chooser.show_all();
        }));
    }

    pub fn run(&self) {
        self.window.show_all();
    }
}

