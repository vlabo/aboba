use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub title: String,
    pub start: i64,
    pub end: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub file: String,
    pub title: String,
    pub chapters: Vec<Chapter>,
    pub time: u64,
}

pub fn get_book(file: &str) -> Book {
    let mut chapters = Vec::new();
    let path = Path::new(file);
    match ffmpeg::format::input(&file) {
        Ok(ictx) => {
            for chapter in ictx.chapters() {
                let title = match chapter.metadata().get("title") {
                    Some(title) => String::from(title),
                    None => String::new(),
                };
                chapters.push(Chapter {
                    title: title,
                    start: chapter.start() / 1000,
                    end: chapter.end() / 1000,
                });
            }
        }

        _ => {}
    }
    let title: &str;
    if let Some(t) = path.file_stem() {
        if let Some(t2) = t.to_str() {
            title = t2;
        } else {
            title = "";
        }
    } else {
        title = "";
    }

    return Book {
        file: file.to_owned(),
        title: title.to_owned(),
        chapters: chapters,
        time: 0,
    };
}

pub fn init_dir(folder: &Path) -> io::Result<Vec<Book>> {
    let mut json_books: Vec<Book> = Vec::new();

    let mut books: Vec<Book> = Vec::new();
    let json_file = folder.join("books.json");
    println!("{}", json_file.to_str().unwrap());
    if let Ok(contents) = fs::read_to_string(json_file.clone()) {
        json_books = serde_json::from_str(&contents).unwrap();
    }

    if folder.is_dir() {
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = &path.extension() {
                if ext.eq(&OsStr::new("m4b")) {
                    if let Some(file_str) = path.to_str() {
                        let book = get_book(&file_str);
                        let mut found = false;
                        for p_book in &json_books {
                            if p_book.title.eq(&book.title) {
                                books.push(p_book.clone());
                                found = true;
                            }
                        }
                        if !found {
                            books.push(book);
                        }
                    }
                }
            }
        }
    }

    if let Ok(mut file) = fs::File::create(json_file) {
        let serialized = serde_json::to_string(&books).unwrap();
        let _ = file.write_all(serialized.as_bytes());
    }
    Ok(books)
}
