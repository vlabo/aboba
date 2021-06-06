use gtk::glib;
use gtk::prelude::*;

use super::super::audio::Player;
use super::super::filemanager::Chapter;

use super::super::util;

pub struct ChaptersView {
    container: gtk::Box,
    back_button: gtk::Button,
    play_chapter_button: gtk::Button,
    list: gtk::ListBox,
}

impl ChaptersView {
    pub fn new() -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let control = gtk::Box::new(gtk::Orientation::Horizontal, 2);

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

        let back_button = gtk::Button::with_label("Back");
        let play_chapter_button = gtk::Button::with_label("Play chapter");
        control.add(&back_button);
        control.add(&play_chapter_button);

        container.add(&control);

        return Self {
            container,
            back_button,
            play_chapter_button,
            list,
        };
    }

    pub fn set_back_fn<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.back_button.connect_clicked(f);
    }

    pub fn get_container(&self) -> &gtk::Box {
        return &self.container;
    }

    pub fn set_chapters(&self, chapters: &Vec<Chapter>, control: Player) {
        for chapter in chapters {
            let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
            container.set_hexpand(true);
            let label_name = gtk::Label::new(Some(&chapter.title));
            container.add(&label_name);
            label_name.set_halign(gtk::Align::Start);
            label_name.set_hexpand(true);
            label_name.set_line_wrap(true);

            let time = util::time_int_to_string((chapter.end - chapter.start) as u64);
            let label_duration = gtk::Label::new(Some(&time.to_string()));
            label_duration.set_halign(gtk::Align::End);
            container.add(&label_duration);
            label_duration.set_hexpand(true);

            self.list.add(&container);
            container.show_all();
        }

        let list = &self.list;
        let back_button = &self.back_button;
        let c = chapters.to_vec();
        self.play_chapter_button.connect_clicked(
            glib::clone!(@weak list, @weak back_button => move |_| {
                if let Some(row) = list.selected_row() {
                    let chapter = &c[row.index() as usize];
                    control.set_position(chapter.start);
                    back_button.emit_clicked();
                }
            }),
        );
    }
}
