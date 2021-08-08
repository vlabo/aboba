mod util;

use std::cell::Cell;
use core::time::Duration;
use std::rc::Rc;
use gtk::glib;
use gtk::prelude::*;

use super::audio::Player;

pub struct Ui {
    window: gtk::Window,
}

impl Ui {
    pub fn build_ui(application: &gtk::Application) -> Self {
        
        let player = if let Some(user_dirs) = directories::UserDirs::new() {
            Rc::new(Player::new(user_dirs.home_dir().join("Audiobooks").as_path()).unwrap())
        } else {
            panic!("Missing Audiobooks folder");
        };


        let inhibit_cookie = Rc::new(Cell::new(0));
        let ui_xml = include_str!("ui.glade");

        let builder = gtk::Builder::from_string(ui_xml);
        let window: gtk::Window = builder.object("MainWindow").expect("Couldn't get MainWindow");
        window.set_application(Some(application));

        // Main Containers
        let main_stack: gtk::Stack = builder.object("MainStack").expect("Couldn't get MainStack");
        let book_list_container: gtk::Box = builder.object("BookListContainer").expect("Couldn't get BookListContainer");
        let player_container: gtk::Box = builder.object("PlayerContainer").expect("Couldn't get PlayerContainer");
        let chapters_container: gtk::Box = builder.object("ChaptersContainer").expect("Couldn't get ChaptersContainer");

        // Book List
        let book_list: gtk::ListBox = builder.object("BookList").expect("Couldn't get BookList");
        let select_book_button: gtk::Button = builder.object("PlayBookButton").expect("Couldn't get PlayBookButton");

        // Player
        let player_books_button: gtk::Button = builder.object("PlayerBooksButton").expect("Couldn't get PlayerBooksButton");
        let player_chapters_button: gtk::Button = builder.object("PlayerChaptersButton").expect("Couldn't get PlayerChaptersButton");
        let player_play_button: gtk::Button = builder.object("PlayButton").expect("Couldn't get PlayButton");
        let player_left_button: gtk::Button = builder.object("PlayerLeftButton").expect("Couldn't get PlayerLeftButton");
        let player_right_button: gtk::Button = builder.object("PlayerRightButton").expect("Couldn't get PlayerRightButton");

        let player_progress_bar: gtk::Scale = builder.object("PlayerProgressBar").expect("Couldn't get PlayerProgressBar");
        let player_progress_label: gtk::Label = builder.object("PlayerProgressLabel").expect("Couldn't get PlayerProgressLabel");

        let player_title_label: gtk::Label = builder.object("PlayerTitleLabel").expect("Couldn't get PlayerTitleLabel");
        let player_chapter_label: gtk::Label = builder.object("PlayerChapterLabel").expect("Couldn't get PlayerChapterLabel");


        // Chapters
        let chapters_back_button: gtk::Button = builder.object("ChaptersBackButton").expect("Couldn't get ChaptersBackButton");
        let chapters_play_button: gtk::Button = builder.object("ChaptersPlayButton").expect("Couldn't get ChaptersPlayButton");
        let chapters_list: gtk::ListBox = builder.object("ChaptersList").expect("Couldn't get ChaptersList");

        // Book List
        select_book_button.connect_clicked(glib::clone!(@weak main_stack, @weak player_container, @weak book_list, @strong player => move |_| {
            if let Some(row) = book_list.selected_row() {
                player.select_book(row.index() as usize);
                main_stack.set_visible_child(&player_container);
            }
        }));

        for title in &player.get_titles() {
            let label = gtk::Label::new(Some(title));
            label.set_wrap(true);
            book_list.add(&label);
            label.show();
        }

        // Player
        player_books_button.connect_clicked(glib::clone!(@weak main_stack, @weak book_list_container, @strong player => move |_| {
            player.pause();
            player.close_book();
            main_stack.set_visible_child(&book_list_container);
        }));

        player_chapters_button.connect_clicked(glib::clone!(@weak main_stack, @weak chapters_container, @weak chapters_list, @strong player => move |_| {
            if let Some(chapters) = player.get_chapters() {
                for child in chapters_list.children() {
                    chapters_list.remove(&child);
                }
                for (title, duration) in chapters {
                    // Container for single chapter
                    let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
                    container.set_hexpand(true);
                    chapters_list.add(&container);

                    // Chapter Name
                    let label_name = gtk::Label::new(Some(&title));
                    label_name.set_halign(gtk::Align::Start);
                    label_name.set_hexpand(true);
                    label_name.set_line_wrap(true);
                    container.add(&label_name);

                    // Chapter duration
                    let time = util::time_int_to_string(duration);
                    let label_duration = gtk::Label::new(Some(&time.to_string()));
                    label_duration.set_halign(gtk::Align::End);
                    label_duration.set_hexpand(true);
                    container.add(&label_duration);

                    container.show_all();
                }
            }

            main_stack.set_visible_child(&chapters_container);
        }));

        player_play_button.connect_clicked(glib::clone!(@weak player_play_button, @weak application, @weak window, @strong player, @strong inhibit_cookie => move |_| {
            if player.is_playing() {
                player_play_button.set_label("Play");
                player.pause();
                application.uninhibit(inhibit_cookie.get());
            } else {
                player_play_button.set_label("Pause");
                player.play();
                inhibit_cookie.set(application.inhibit(Some(&window), gtk::ApplicationInhibitFlags::SUSPEND, Some("Aboba is playing.")));
            }
        }));

        player_left_button.connect_clicked(glib::clone!(@strong player => move |_| {
            let position = player.get_position() - 30;
            player.set_position(position);
        }));

        player_right_button.connect_clicked(glib::clone!(@strong player => move |_| {
            let position = player.get_position() + 30;
            player.set_position(position);
        }));

        // Chapters

        chapters_back_button.connect_clicked(glib::clone!(@weak main_stack, @weak player_container => move |_| {
            main_stack.set_visible_child(&player_container);
        }));

        chapters_play_button.connect_clicked(glib::clone!(@weak main_stack, @weak player_container, @weak chapters_list, @strong player => move |_| {
            if let Some(row) = chapters_list.selected_row() {
                player.set_chapter(row.index() as usize);
            }
            main_stack.set_visible_child(&player_container);
        }));


        glib::timeout_add_local(
            Duration::from_millis(500),
            glib::clone!(@weak player_progress_bar, @weak player_progress_label, @weak player_title_label, @weak player_chapter_label, @strong player => @default-return glib::Continue(false), move || {
                if let Some((duration, position)) = player.get_current_chapter_duration_and_position() {
                    player_progress_bar.set_range(0.0, duration as f64);
                    player_progress_bar.set_value(position as f64);
                    let lable_text = util::time_int_to_string(position) + "/" + &util::time_int_to_string(duration);
                    player_progress_label.set_text(&lable_text);

                    if let Some(title) = player.get_current_book_title() {
                        player_title_label.set_text(&title);
                    } else {
                        player_title_label.set_text("");
                    }

                    if let Some(title) = player.get_current_chapter_title() {
                        player_chapter_label.set_text(&title);
                    } else {
                        player_chapter_label.set_text("");
                    }
                }

                glib::Continue(true)
            }),
        );

        player_progress_bar.connect_change_value(glib::clone!(@strong player => @default-return gtk::Inhibit(false), move |_, _, value| {
            if let Some((start, _)) = player.get_current_chapter_start_and_end() {
                let new_position = start + value as i64;
                player.set_position(new_position);
            }

            gtk::Inhibit(false)
        }));

        // Window

        window.connect_delete_event(glib::clone!(@strong player => @default-return gtk::Inhabit(false), move |_, _| {
            player.close_book();
            gtk::Inhibit(false)
        }));

        return Ui {
            window
        };
    }


    pub fn run(&self) {
        self.window.show();
    }
}
