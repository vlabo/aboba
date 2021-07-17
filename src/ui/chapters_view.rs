use gtk::glib;
use gtk::prelude::*;

use super::super::file_manager::Chapter;
use super::player_view::PlayerView;

use super::super::util;

pub struct ChaptersView {
    container: gtk::Box,
    back_button: gtk::Button,
    play_chapter_button: gtk::Button,
    chapter_list: gtk::ListBox,
}

impl ChaptersView {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);

        // Scrolled window
        let scrolled_window =
            gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_shadow_type(gtk::ShadowType::EtchedIn);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        scrolled_window.set_vexpand(true);
        container.add(&scrolled_window);

        // List Viewport
        let viewport = gtk::Viewport::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        viewport.set_vexpand(true);
        scrolled_window.add(&viewport);

        // Chapters List
        let chapter_list = gtk::ListBox::new();
        viewport.add(&chapter_list);

        // Buttons Container
        let control = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        container.add(&control);

        // Back Button
        let back_button = gtk::Button::with_label("Back");
        control.add(&back_button);

        // Play chapter button
        let play_chapter_button = gtk::Button::with_label("Play chapter");
        control.add(&play_chapter_button);

        return Self {
            container,
            back_button,
            play_chapter_button,
            chapter_list,
        };
    }

    pub fn set_back_fn<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.back_button.connect_clicked(f);
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn set_chapters(&self, chapters: &Vec<Chapter>, player_view: std::rc::Rc<PlayerView>) {
        for chapter in chapters {
            // Container for single chapter
            let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
            container.set_hexpand(true);
            self.chapter_list.add(&container);

            // Chapter Name
            let label_name = gtk::Label::new(Some(&chapter.title));
            label_name.set_halign(gtk::Align::Start);
            label_name.set_hexpand(true);
            label_name.set_line_wrap(true);
            container.add(&label_name);

            // Chapter duration
            let time = util::time_int_to_string(chapter.duration);
            let label_duration = gtk::Label::new(Some(&time.to_string()));
            label_duration.set_halign(gtk::Align::End);
            label_duration.set_hexpand(true);
            container.add(&label_duration);

            container.show_all();
        }

        let chapter_list = &self.chapter_list;
        let back_button = &self.back_button;
        let chapters = chapters.to_vec();
        self.play_chapter_button.connect_clicked(
            glib::clone!(@weak chapter_list, @weak back_button, @weak player_view => move |_| {
                if let Some(row) = chapter_list.selected_row() {
                    let chapter = &chapters[row.index() as usize];
                    player_view.set_position(chapter.start);
                    back_button.emit_clicked();
                }
            }),
        );
    }
}
