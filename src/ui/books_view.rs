use super::super::file_manager::{Book, Books};
use gtk::glib;
use gtk::prelude::*;

use std::cell::Cell;
use std::rc::Rc;

pub struct BooksView {
    container: gtk::Box,
    book_list: gtk::ListBox,
    book_info_list: Rc<Cell<Option<Books>>>,
    play_button: gtk::Button,
}

impl BooksView {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);

        // Window For Scrolling
        let scrolled_window =
            gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_shadow_type(gtk::ShadowType::EtchedIn);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        scrolled_window.set_vexpand(true);
        container.add(&scrolled_window);

        // View port
        let viewport = gtk::Viewport::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        viewport.set_vexpand(true);
        scrolled_window.add(&viewport);

        // List of books
        let book_list = gtk::ListBox::new();
        viewport.add(&book_list);

        // Play Button
        let play_button = gtk::Button::with_label("Play");
        container.add(&play_button);

        // Objects
        let book_info_list = Rc::new(Cell::new(None));

        return Self {
            container,
            book_list,
            book_info_list,
            play_button,
        };
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn add_book_list(&self, books: Books) {
        for child in self.book_list.children() {
            self.book_list.remove(&child);
        }

        for book in &books.list {
            let label = gtk::Label::new(Some(&book.title));
            label.set_line_wrap(true);
            self.book_list.add(&label);
            label.show();
        }
        self.book_list.unselect_all();
        self.book_info_list.set(Some(books));
    }

    pub fn get_books(&self) -> Option<Books> {
        self.book_info_list.take()
    }

    pub fn connect_book_selected<F: Fn(Book) + 'static>(&self, f: F) {
        let book_info_list = &self.book_info_list;
        let book_list = &self.book_list;
        self.play_button.connect_clicked(
            glib::clone!(@weak book_info_list, @weak book_list => move |_| {
                if let Some(row) = book_list.selected_row() {
                    if let Some(books) = book_info_list.take() {
                        f(books.list[row.index() as usize].clone());
                        book_info_list.set(Some(books));
                    }
                }
            }),
        );
    }
}
