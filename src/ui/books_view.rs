use super::super::filemanager::{Book, Books};
use gtk::glib;
use gtk::prelude::*;

use std::cell::Cell;
use std::rc::Rc;

pub struct BooksView {
    container: gtk::Box,
    list: gtk::ListBox,
    books_list: Rc<Cell<Option<Books>>>,
}

impl BooksView {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);

        let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        sw.set_shadow_type(gtk::ShadowType::EtchedIn);
        sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        sw.set_vexpand(true);

        let viewport = gtk::Viewport::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        viewport.set_vexpand(true);
        let list = gtk::ListBox::new();

        viewport.add(&list);
        sw.add(&viewport);
        sw.set_vexpand(true);
        container.add(&sw);

        let books_list = Rc::new(Cell::new(None));
        return Self {
            container,
            list,
            books_list,
        };
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn add_book_list(&self, books: Books) {
        for book in &books.list {
            let label = gtk::Label::new(Some(&book.title));
            self.list.add(&label);
            label.show();
        }
        self.books_list.set(Some(books));
    }

    pub fn get_books(&self) -> Option<Books> {
        self.books_list.take()
    }

    pub fn connect_book_selected<F: Fn(Book) + 'static>(&self, f: F) {
        let books_list = &self.books_list;
        self.list
            .connect_row_selected(glib::clone!(@weak books_list => move |_, row| {
                if let Some(row) = row {
                    if let Some(books) = books_list.take() {
                        f(books.list[row.index() as usize].clone());
                        books_list.set(Some(books));
                    }
                }
            }));
    }
}
