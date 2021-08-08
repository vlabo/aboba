mod audio_player;
mod c;

use std::cell::Cell;
use audio_player::AudioPlayer;
use super::file_manager::{init_dir, Book, Chapter};
use anyhow::Result;
use std::path::Path;


pub struct Player {
    folder: String,
    audio_player: Cell<Option<AudioPlayer>>,
    books: Cell<Vec<Book>>,
    current_book: Cell<Option<Book>>,
}

impl Player {
    pub fn new(folder: &Path) -> Result<Player, anyhow::Error> {
        let books = init_dir(folder)?;

        return Ok(Player {
            folder: folder.to_str().unwrap().to_string(),
            audio_player: Cell::new(None),
            books: Cell::new(books),
            current_book: Cell::new(None),
        });
    }

    pub fn get_titles(&self) -> Vec<String>{
        let mut titles = Vec::new();

        let books = self.books.take();
        for book in &books {
            titles.push(book.title.to_string());
        }
        self.books.set(books);

        return titles;
    }

    pub fn select_book(&self, index: usize) {
        let books = self.books.take();
        if index < books.len() {
            let player = AudioPlayer::new();
            let book = books[index].clone();
            let path =  Path::new(&book.file);
            player.set_file(path);
            player.set_position(book.time);
            self.audio_player.set(Some(player));
            self.current_book.set(Some(book));
        }
        self.books.set(books);
    }

    pub fn is_playing(&self) -> bool {
        if let Some(player) = self.audio_player.take() {
            let is_playing = player.is_playing();
            self.audio_player.set(Some(player));
            return is_playing;
        } else {
            return false;
        }
        
    }

    pub fn play(&self) {
        if let Some(player) = self.audio_player.take() {
            player.play();
            self.audio_player.set(Some(player));
        }
    }

    pub fn pause(&self) {
        if let Some(player) = self.audio_player.take() {
            player.pause();
            self.audio_player.set(Some(player));
        }
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> i64 {
        if let Some(player) = self.audio_player.take() {
            let duration = player.get_duration();
            self.audio_player.set(Some(player));
            return duration;
        } else {
            return 0;
        }
    }

    pub fn get_position(&self) -> i64 {
        if let Some(player) = self.audio_player.take() {
            let position = player.get_position();
            self.audio_player.set(Some(player));
            return position;
        } else {
            return 0;
        }
    }

    pub fn set_position(&self, position: i64) {
        if let Some(player) = self.audio_player.take() {
            player.set_position(position);
            self.audio_player.set(Some(player));
        }
    }

    pub fn get_chapters(&self) -> Option<Vec<(String, i64)>> {
        if let Some(book) = self.current_book.take() {
            let mut chapters = Vec::new();
            for chapter in &book.chapters {
                chapters.push((chapter.title.to_string(), chapter.duration));
            }
            self.current_book.replace(Some(book));
            return Some(chapters);
        }
        return None;
    }

    pub fn set_chapter(&self, index: usize) {
        if let Some(book) = self.current_book.take() {
            if index < book.chapters.len() {
                self.set_position(book.chapters[index].start);
            }
            self.current_book.set(Some(book));
        }
    }

    pub fn get_current_book_title(&self) -> Option<String> {
        if let Some(book) = self.current_book.take() {
            let title = String::from(&book.title);
            self.current_book.set(Some(book));
            return Some(title);
        } else {
            return None;
        }
    }
    pub fn get_current_chapter_title(&self) -> Option<String> {
        if let Some(chapter) = self.get_current_chapter() {
            return Some(chapter.title.to_string());
            
        }
        return None;
    }

    pub fn get_current_chapter_duration_and_position(&self) -> Option<(i64, i64)> {
        if let Some(chapter) = self.get_current_chapter() {
            let position = self.get_position();
            return Some((chapter.duration, position - chapter.start));
        }
        return None;
    }

    pub fn get_current_chapter_start_and_end(&self) -> Option<(i64, i64)> {
        if let Some(chapter) = self.get_current_chapter() {
            return Some((chapter.start, chapter.end));
        }
        return None;
    }

    fn get_current_chapter(&self) -> Option<Chapter> {
        if let Some(book) = self.current_book.take() {
            let mut current_chapter = None;
            if let Some(player) = self.audio_player.take() {
                let position = player.get_position();
                for chapter in &book.chapters {
                    if position >= chapter.start && position < chapter.end {
                        current_chapter = Some(chapter.clone());
                        break;
                    }
                }
                self.audio_player.set(Some(player));
            }
            self.current_book.set(Some(book));
            return current_chapter;
        }
        return None;   
    }

    pub fn close_book(&self) {
        if let Some(current_book) = self.current_book.take() {
            let mut books = self.books.take();
            if let Some(player) = self.audio_player.take() {
                for i in 0..books.len() {
                    if books[i].title.eq(&current_book.title) {
                        books[i].time = player.get_position();
                        break;
                    }
                }
            }
            super::file_manager::save_json(&self.folder, &books);
            self.books.set(books);
        }
    }

}