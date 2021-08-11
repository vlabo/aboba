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
#[derive(Clone)]
struct BookListView {
    container: gtk::Box,
    list: gtk::ListBox,
    select_button: gtk::Button,
}

impl BookListView {
    fn fill_titles(&self, titles: &Vec<String>) {
        for title in titles {
            let label = gtk::Label::new(Some(title));
            label.set_widget_name("book_label");
            label.set_wrap(true);
            self.list.add(&label);
            label.show();
        }
    }
}

#[derive(Clone)]
struct PlayerView {
    container: gtk::Box,
    books_button: gtk::Button,
    chapters_button: gtk::Button,
    play_button: gtk::Button,
    left_button: gtk::Button,
    right_button: gtk::Button,
    progress_bar: gtk::Scale,
    progress_label: gtk::Label,
    title_label: gtk::Label,
    chapter_label: gtk::Label,
}

impl PlayerView {
    fn update(&self, player: &Player) {

        self.update_time(player);
        if let Some(title) = player.get_current_book_title() {
            self.title_label.set_text(&title);
        } else {
            self.title_label.set_text("");
        }

        if let Some(title) = player.get_current_chapter_title() {
            self.chapter_label.set_text(&title);
        } else {
            self.chapter_label.set_text("");
        }
    
    }

    fn update_time(&self, player: &Player) {
        if let Some((duration, position)) = player.get_current_chapter_duration_and_position() {
            self.progress_bar.set_range(0.0, duration as f64);
            self.progress_bar.set_value(position as f64);
            let lable_text = util::time_int_to_string(position) + "/" + &util::time_int_to_string(duration);
            self.progress_label.set_text(&lable_text);
        }
    }
}

#[derive(Clone)]
struct ChaptersView {
    container: gtk::Box,
    back_button: gtk::Button,
    play_button: gtk::Button,
    list: gtk::ListBox,
}

impl ChaptersView {
    fn fill_chapters(&self, chapters: Vec<(String, i64)>) {
        for child in self.list.children() {
            self.list.remove(&child);
        }
        for (title, duration) in &chapters {
            // Container for single chapter
            let container = gtk::Box::new(gtk::Orientation::Horizontal, 2);
            container.set_widget_name("chapter_label");
            container.set_hexpand(true);
            self.list.add(&container);

            // Chapter Name
            let label_name = gtk::Label::new(Some(title));
            label_name.set_halign(gtk::Align::Start);
            label_name.set_hexpand(true);
            label_name.set_line_wrap(true);
            container.add(&label_name);

            // Chapter duration
            let time = util::time_int_to_string(*duration);
            let label_duration = gtk::Label::new(Some(&time));
            label_duration.set_halign(gtk::Align::End);
            label_duration.set_hexpand(true);
            container.add(&label_duration);

            container.show_all();
        }
    }
}

fn initialize_views(builder: &gtk::Builder) -> (Rc<BookListView>, Rc<PlayerView>, Rc<ChaptersView>){
    let book_list_view = BookListView {
        container: builder.object("BookListContainer").expect("Couldn't get BookListContainer"),
        list: builder.object("BookList").expect("Couldn't get BookList"),
        select_button: builder.object("PlayBookButton").expect("Couldn't get PlayBookButton"),
    };

    let player_view = PlayerView {
        container: builder.object("PlayerContainer").expect("Couldn't get PlayerContainer"),
        books_button: builder.object("PlayerBooksButton").expect("Couldn't get PlayerBooksButton"),
        chapters_button: builder.object("PlayerChaptersButton").expect("Couldn't get PlayerChaptersButton"),
        play_button: builder.object("PlayButton").expect("Couldn't get PlayButton"),
        left_button: builder.object("PlayerLeftButton").expect("Couldn't get PlayerLeftButton"),
        right_button: builder.object("PlayerRightButton").expect("Couldn't get PlayerRightButton"),
        progress_bar: builder.object("PlayerProgressBar").expect("Couldn't get PlayerProgressBar"),
        progress_label: builder.object("PlayerProgressLabel").expect("Couldn't get PlayerProgressLabel"),
        title_label: builder.object("PlayerTitleLabel").expect("Couldn't get PlayerTitleLabel"),
        chapter_label: builder.object("PlayerChapterLabel").expect("Couldn't get PlayerChapterLabel"),
    };

    let chapters_view = ChaptersView {
        container: builder.object("ChaptersContainer").expect("Couldn't get ChaptersContainer"),
        back_button: builder.object("ChaptersBackButton").expect("Couldn't get ChaptersBackButton"),
        play_button: builder.object("ChaptersPlayButton").expect("Couldn't get ChaptersPlayButton"),
        list: builder.object("ChaptersList").expect("Couldn't get ChaptersList"),
    };

    return (Rc::new(book_list_view), Rc::new(player_view), Rc::new(chapters_view));
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

        let (book_list_view, player_view, chapters_view) = initialize_views(&builder);

        // Book List
        book_list_view.select_button.connect_clicked(glib::clone!(@weak main_stack, @strong player_view, @strong book_list_view, @strong player => move |_| {
            if let Some(row) = book_list_view.list.selected_row() {
                player.select_book(row.index() as usize);
                player_view.update(&player);
                main_stack.set_visible_child(&player_view.container);
            }
        }));

        book_list_view.fill_titles(&player.get_titles());

        // Player
        player_view.books_button.connect_clicked(glib::clone!(@weak main_stack, @strong book_list_view, @strong player => move |_| {
            player.pause();
            player.close_book();
            main_stack.set_visible_child(&book_list_view.container);
        }));

        player_view.chapters_button.connect_clicked(glib::clone!(@weak main_stack, @strong chapters_view, @strong player => move |_| {
            if let Some(chapters) = player.get_chapters() {
                chapters_view.fill_chapters(chapters);
            }

            main_stack.set_visible_child(&chapters_view.container);
        }));

        player_view.play_button.connect_clicked(glib::clone!(@strong player_view, @weak application, @weak window, @strong player, @strong inhibit_cookie => move |_| {
            if player.is_playing() {
                player_view.play_button.set_label("Play");
                player.pause();
                application.uninhibit(inhibit_cookie.get());
            } else {
                player_view.play_button.set_label("Pause");
                player.play();
                inhibit_cookie.set(application.inhibit(Some(&window), gtk::ApplicationInhibitFlags::SUSPEND, Some("Aboba is playing.")));
            }
        }));

        player_view.left_button.connect_clicked(glib::clone!(@strong player => move |_| {
            let mut position = player.get_position() - 30;
            if position < 0 {
                position = 0;
            }
            player.set_position(position);
        }));

        player_view.right_button.connect_clicked(glib::clone!(@strong player => move |_| {
            let mut position = player.get_position() + 30;
            let duration = player.get_duration();
            if position > duration{
                position = duration;
            }
            player.set_position(position);
        }));

        // Chapters

        chapters_view.back_button.connect_clicked(glib::clone!(@weak main_stack, @strong player_view => move |_| {
            main_stack.set_visible_child(&player_view.container);
        }));

        chapters_view.play_button.connect_clicked(glib::clone!(@weak main_stack, @strong player_view, @strong chapters_view, @strong player => move |_| {
            if let Some(row) = chapters_view.list.selected_row() {
                player.set_chapter(row.index() as usize);
                player_view.update(&player);
            }
            main_stack.set_visible_child(&player_view.container);
        }));


        glib::timeout_add_local(
            Duration::from_millis(500),
            glib::clone!(@strong player_view, @strong player, @weak window => @default-return glib::Continue(false), move || {
                // if player.is_playing()  {
                    player_view.update(&player);
                // }
                glib::Continue(true)
            }),
        );

        player_view.progress_bar.connect_change_value(glib::clone!(@strong player_view, @strong player => @default-return gtk::Inhibit(false), move |_, _, value| {
            if let Some((start, _)) = player.get_current_chapter_start_and_end() {
                let new_position = start + value as i64;
                player.set_position(new_position);
                player_view.update_time(&player);
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
