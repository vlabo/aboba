use super::super::filemanager::Book;
use gtk::prelude::*;

pub struct BooksView {
    container: gtk::Box,
    list: gtk::ListBox,
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
        return Self { container, list };
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn add_book_list(&self, book_list: &Vec<Book>) {
        for book in book_list {
            let label = gtk::Label::new(Some(&book.title));
            self.list.add(&label);
            label.show();
        }
    }

    pub fn connect_book_selected<F: Fn(usize) + 'static>(&self, f: F) {
        self.list.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                f(row.index() as usize);
            }
        });
    }
}
