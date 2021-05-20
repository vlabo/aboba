use gtk::glib;
use gtk::prelude::*;

mod books_view;
mod chapters_view;
mod player_view;

use books_view::*;
use chapters_view::*;
use player_view::*;

use std::cell::Cell;
use std::rc::Rc;

use super::filemanager::{self, Book, Books};

pub struct Ui {
    window: gtk::Window,
    books_view: Rc<BooksView>,
    player_view: Rc<PlayerView>,
    chapters_view: Rc<ChaptersView>,
    open_button: gtk::Button,
    stack: gtk::Stack,
}

impl Ui {
    pub fn build_ui(application: &gtk::Application) -> Self {
        let builder = gtk::WindowBuilder::new();
        let window = builder
            .application(application)
            .width_request(300)
            .height_request(500)
            .build();

        window.set_border_width(0);
        window.set_position(gtk::WindowPosition::Center);

        // Header
        let bar = gtk::HeaderBar::new();
        bar.set_show_close_button(true);
        let open_button = gtk::Button::with_label("Open");
        bar.add(&open_button);
        window.set_titlebar(Some(&bar));

        // Player
        let player_view = PlayerView::new();
        let player_container = player_view.get_container();

        // Chapters
        let chapters_view = ChaptersView::new();
        let chapters_container = chapters_view.get_container();

        // Books list
        let books_view = BooksView::new();
        let _books_container = books_view.get_container();

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);

        stack.add(books_view.get_container());
        stack.add(player_view.get_container());
        stack.add(chapters_view.get_container());

        chapters_view.set_back_fn(
            glib::clone!(@weak stack, @weak player_container => move |_| {
                stack.set_visible_child(&player_container);
            }),
        );

        player_view.set_open_chapters_fn(
            glib::clone!(@weak stack, @weak chapters_container => move |_| {
                stack.set_visible_child(&chapters_container);
            }),
        );

        window.add(&stack);

        //let books_list = Rc::new(Cell::new(None));

        return Self {
            window,
            books_view: Rc::new(books_view),
            player_view: Rc::new(player_view),
            chapters_view: Rc::new(chapters_view),
            open_button,
            stack,
        };
    }

    pub fn setup_open_button(&self, get_control: &'static dyn Fn(&str) -> super::audio::Control) {
        let open_button = &self.open_button;
        let window = &self.window;
        let books = self.books_view.clone();

        open_button.connect_clicked(glib::clone!(@weak window => move |_| {

            let file_chooser = gtk::FileChooserDialog::new(
                Some("Open"),
                Some(&window),
                gtk::FileChooserAction::SelectFolder,
            );
            file_chooser.add_buttons(&[
                ("Select", gtk::ResponseType::Ok),
                ("Cancel", gtk::ResponseType::Cancel),
            ]);
            let books = books.clone();

            file_chooser.connect_response(glib::clone!(@strong books => move |file_chooser, response| {
                if response == gtk::ResponseType::Ok {
                    let dir = file_chooser.filename().expect("Couldn't get filename");
                    if let Ok(list) = super::filemanager::init_dir(dir.as_path()) {
                        books.add_book_list(list);
                    }

                }
                file_chooser.close();
            }));

            file_chooser.show_all();
        }));

        let stack = &self.stack;
        let chapters_view = self.chapters_view.clone();
        let player_view = self.player_view.clone();
        self.books_view.connect_book_selected(
            glib::clone!(@weak stack, @strong chapters_view, @strong player_view => move |book| {
                println!("{}", &book.title);
                let control = get_control(&book.file);
                let _ = control.is_playing();
                chapters_view.set_chapters(&book.chapters, control.clone());
                player_view.initialize_book(book, control);
                stack.set_visible_child(player_view.get_container());
            }),
        );

        let player_view = self.player_view.clone();
        let books = self.books_view.clone();
        window.connect_delete_event(glib::clone!(@strong books, @strong player_view => @default-return gtk::Inhabit(false), move |_, _| {
            if let Some(mut books) = books.get_books() {
                if let Some(book) = player_view.get_book() {
                    for i in 0..books.list.len() {
                        if books.list[i].title.eq(&book.title) {
                            println!("{}", &book.time);
                            books.list[i] = book;
                            break;
                        }
                    }
                    filemanager::save_json(&books);
                }
            }
            gtk::Inhibit(false)
        }));
    }

    pub fn run(&self) {
        self.window.show_all();
    }
}
