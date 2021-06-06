use super::super::audio::Player;
use super::super::file_manager::{Book, Chapter};
use super::super::util;
use gtk::glib;
use gtk::prelude::*;
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

pub struct PlayerView {
    container: gtk::Box,
    chapters_button: gtk::Button,
    play_button: gtk::Button,
    seek_back_button: gtk::Button,
    book_title: gtk::Label,
    chapter_title: gtk::Label,
    progress_bar: gtk::Scale,
    progress_label: gtk::Label,
    book_info: Rc<RefCell<Book>>,
    player: Player,
}

impl PlayerView {
    pub fn new() -> Self {
        // Parent container
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        // Progress Label
        let progress_label = gtk::Label::new(None);
        progress_label.set_hexpand(true);
        container.add(&progress_label);

        // Progress Bar
        let progress_bar = gtk::Scale::new(
            gtk::Orientation::Horizontal,
            Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)),
        );
        container.add(&progress_bar);

        // Book Title
        let book_title = gtk::Label::new(Some("Title"));
        book_title.set_hexpand(true);
        book_title.set_vexpand(true);
        container.add(&book_title);

        // Chapter Title
        let chapter_title = gtk::Label::new(Some("Chapter"));
        chapter_title.set_hexpand(true);
        chapter_title.set_vexpand(true);
        container.add(&chapter_title);

        // Button Containers
        let control_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 3);
        control_box.add(&button_box);
        container.add(&control_box);

        // Seek back Button
        let seek_back_button = gtk::Button::with_label("< 30s");
        seek_back_button.set_hexpand(true);
        button_box.add(&seek_back_button);

        // Play Button
        let play_button = gtk::Button::with_label("Play");
        play_button.set_hexpand(true);
        button_box.add(&play_button);

        // Chapters Button
        let chapters_button = gtk::Button::with_label("Chapters");
        chapters_button.set_hexpand(true);
        button_box.add(&chapters_button);

        // Objects
        let book_info = Rc::new(RefCell::new(Book::default()));
        let player = Player::new();

        return Self {
            container,
            chapters_button,
            play_button,
            seek_back_button,
            book_title,
            chapter_title,
            progress_bar,
            progress_label,
            book_info,
            player,
        };
    }

    pub fn set_open_chapters_fn<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.chapters_button.connect_clicked(f);
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn initialize_book(&self, book: Book) {
        self.player.set_file(std::path::Path::new(&book.file));
        self.book_title.set_label(&book.title);
        self.book_info.replace(book);
        self.init_play_button();
        self.init_progress_bar();
        self.init_seek_back_button();
    }

    fn init_play_button(&self) {
        let book = self.book_info.clone();
        let player = self.player.clone();
        self.play_button.connect_clicked(move |play_button| {
            if player.is_playing() {
                player.pause();
                book.borrow_mut().time = player.get_position();
                play_button.set_label("Play");
            } else {
                player.play();
                player.set_position(book.borrow().time);
                play_button.set_label("Pause");
            }
        });
    }

    fn init_progress_bar(&self) {
        let progress_bar = &self.progress_bar;
        let chapter_title = &self.chapter_title;
        let progress_label = &self.progress_label;

        let book = self.book_info.clone();
        let player = self.player.clone();

        if let Some(i) = Self::get_current_chapter(&book.borrow().chapters, book.borrow().time) {
            let chapter = &book.borrow().chapters[i];
            chapter_title.set_label(&chapter.title);
            Self::update_progress_bar(&progress_bar, &progress_label, book.borrow().time - chapter.start, chapter.duration);
        }

        glib::timeout_add_local(
            Duration::from_millis(500),
            glib::clone!(@weak progress_bar, @weak chapter_title, @weak progress_label, @weak book => @default-return glib::Continue(false), move || {
                if player.is_playing() {
                    let position = player.get_position();

                    if let Some(i) = Self::get_current_chapter(&book.borrow().chapters, position) {
                        let chapter = &book.borrow().chapters[i];
                        chapter_title.set_label(&chapter.title);
                        Self::update_progress_bar(&progress_bar, &progress_label, position - chapter.start, chapter.duration);
                    }
                    book.borrow_mut().time = position;
                }
                glib::Continue(true)
            }),
        );

        let book = self.book_info.clone();
        let player = self.player.clone();

        progress_bar.connect_change_value(move |_, _, v| {
            let position = player.get_position();
            let mut book = book.borrow_mut();
            if let Some(i) = Self::get_current_chapter(&book.chapters, position) {
                let chapter = &book.chapters[i];
                let new_position = chapter.start + v as i64;
                player.set_position(new_position);
                book.time = new_position;
            }

            gtk::Inhibit(false)
        });

        progress_bar.connect_format_value(|_, _| {
            return String::new();
        });
    }

    fn update_progress_bar(progress_bar: &gtk::Scale, progress_label: &gtk::Label, position: i64, chapter_duration: i64) {
        progress_bar.set_range(0.0, chapter_duration as f64);
        progress_bar.set_value(position as f64);
        progress_label.set_text(&util::time_int_to_string(position));
    }

    fn init_seek_back_button(&self) {
        let player = self.player.clone();
        self.seek_back_button.connect_clicked(move |_| {
            let position = player.get_position();
            if position > 30 {
                player.set_position(position - 30);
            } else {
                player.set_position(0);
            }
        });
    }

    fn get_current_chapter(chapters: &Vec<Chapter>, position: i64) -> Option<usize> {
        for i in 0..chapters.len() {
            let chapter = &chapters[i];
            if position >= chapter.start && position < chapter.end {
                return Some(i);
            }
        }
        return None;
    }

    pub fn get_book(&self) -> Option<Book> {
        if self.book_info.borrow().file.len() == 0 {
            return None;
        }

        return Some(self.book_info.borrow().clone());
    }

    pub fn set_position(&self, position: i64) {
        let mut book_info = self.book_info.borrow_mut();
        if let Some(i) = Self::get_current_chapter(&book_info.chapters, position) {
            let chapter = &book_info.chapters[i];
            self.chapter_title.set_label(&chapter.title);
            Self::update_progress_bar(&self.progress_bar, &self.progress_label, position - chapter.start, chapter.duration);
        }
        self.player.set_position(position);
        book_info.time = position;
    }
}
