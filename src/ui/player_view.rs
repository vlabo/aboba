use super::super::audio::Control;
use gtk::glib;
use gtk::prelude::*;
use std::time::Duration;
use super::super::filemanager::Chapter;

pub struct PlayerView {
    container: gtk::Box,
    chapters_button: gtk::Button,
    play_button: gtk::Button,
    play_back_button: gtk::Button,
    title: gtk::Label,
    chapter: gtk::Label,
    progress_bar: gtk::Scale,
}

impl PlayerView {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let progress_bar = gtk::Scale::new(
            gtk::Orientation::Horizontal,
            Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)),
        );
        container.add(&progress_bar);

        let title = gtk::Label::new(Some("Title"));
        title.set_hexpand(true);
        title.set_vexpand(true);
        container.add(&title);

        let chapter = gtk::Label::new(Some("Chapter"));
        chapter.set_hexpand(true);
        chapter.set_vexpand(true);
        container.add(&chapter);

        let control_box = gtk::Box::new(gtk::Orientation::Vertical, 3);

        let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 3);
        let play_back_button = gtk::Button::with_label("< 30s");
        play_back_button.set_hexpand(true);
        button_box.add(&play_back_button);

        let play_button = gtk::Button::with_label("Play");
        play_button.set_hexpand(true);

        button_box.add(&play_button);
        let chapters_button = gtk::Button::with_label("Chapters");
        chapters_button.set_hexpand(true);
        button_box.add(&chapters_button);

        control_box.add(&button_box);
        container.add(&control_box);

        return Self {
            container,
            chapters_button,
            play_button,
            play_back_button,
            title,
            chapter,
            progress_bar,
        };
    }

    pub fn set_open_chapters_fn<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.chapters_button.connect_clicked(f);
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn initialize_book(&self, title: &str, chapters: &Vec<Chapter>, control: Control) {
        self.title.set_label(title);
        self.init_play_button(control.clone());
        self.init_progress_bar(control.clone(), chapters);
        self.init_play_back_button(control);
    }

    fn init_play_button(&self, control: Control) {
        self.play_button.connect_clicked(move |b| {
            if control.is_playing() {
                let _ = control.pause();
                b.set_label("Play");
            } else {
                let _ = control.play();
                b.set_label("Pause");
            }
        });
    }

    fn init_progress_bar(&self, control: Control, chapters: &Vec<Chapter>) {
        let progress_bar = &self.progress_bar;
        let manual_control = control.clone();
        let chapter_title = &self.chapter;
        let chaps = chapters.to_vec();
        glib::timeout_add_local(Duration::from_millis(500), glib::clone!(@weak progress_bar, @weak chapter_title => @default-return glib::Continue(false), move || {
            if control.is_playing() {
                let position = control.get_position() as i64;
                if let Some(i) = Self::get_current_chapter(&chaps, position) {
                    let chapter = &chaps[i];
                    progress_bar.set_range(0.0, (chapter.end - chapter.start) as f64);
                    progress_bar.set_value((position - chapter.start) as f64);
                    chapter_title.set_label(&chapter.title);
                }
            }
            glib::Continue(true)
        }));

        let chaps2 = chapters.to_vec();
        progress_bar.connect_change_value( move |_, _, v| {
            let position = manual_control.get_position() as i64;
            if let Some(i) = Self::get_current_chapter(&chaps2, position) {
                let chapter = &chaps2[i];
                manual_control.set_position((chapter.start + v as i64) as u64);
            }

            gtk::Inhibit(false)
        });
    }

    fn init_play_back_button(&self, control: Control) {
        self.play_back_button.connect_clicked(move |_| {
            let position = control.get_position();
            if position > 30 {
                control.set_position(position - 30);
            } else {
                control.set_position(0);
            }
        });
    }

    fn get_current_chapter(chapters: &Vec<Chapter>, position: i64) -> Option<usize> {
        for i in 0..chapters.len() {
            let chapter = &chapters[i];
            if position > chapter.start && position < chapter.end {
                return Some(i);
            }
        }
        return None;
    }
}