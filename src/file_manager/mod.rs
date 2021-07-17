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
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Book {
    pub file: String,
    pub title: String,
    pub chapters: Vec<Chapter>,
    pub time: i64,
}

pub struct Books {
    pub path: String,
    pub list: Vec<Book>,
}

pub fn get_book(file: &str) -> Book {
    let chapters = Vec::new();
    let path = Path::new(file);
    // match ffmpeg::format::input(&file) {
    //     Ok(ictx) => {
    //         for chapter in ictx.chapters() {
    //             let mut title = match chapter.metadata().get("title") {
    //                 Some(title) => String::from(title),
    //                 None => String::new(),
    //             };

    //             if title.trim().len() == 0 {
    //                 title = format!("Chapter {}", chapter.index());
    //             }

    //             let start = chapter.start() * chapter.time_base().0 as i64 / chapter.time_base().1 as i64;
    //             let end = chapter.end() * chapter.time_base().0 as i64 / chapter.time_base().1 as i64;

    //             chapters.push(Chapter {
    //                 title: title,
    //                 start: start,
    //                 end: end,
    //                 duration: end - start,
    //             });
    //         }
    //     }

    //     _ => {}
    // }
    let mut title: &str = "";
    if let Some(file_stem) = path.file_stem() {
        if let Some(file_name) = file_stem.to_str() {
            title = file_name;
        }
    }

    return Book {
        file: file.to_owned(),
        title: title.to_owned(),
        chapters: chapters,
        time: 0,
    };
}

pub fn init_dir(folder: &Path) -> io::Result<Books> {
    let mut json_books: Vec<Book> = Vec::new();

    let mut books: Vec<Book> = Vec::new();
    let json_file = folder.join("books.json");

    if let Ok(contents) = fs::read_to_string(json_file.clone()) {
        json_books = serde_json::from_str(&contents).unwrap();
    }

    if folder.is_dir() {
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = &path.extension() {
                if ext.eq(&OsStr::new("m4a"))
                    || ext.eq(&OsStr::new("m4b"))
                    || ext.eq(&OsStr::new("3gp"))
                {
                    if let Some(file_str) = path.to_str() {
                        let book = get_book(&file_str);
                        let mut found = false;
                        for p_book in &json_books {
                            if p_book.file.eq(&book.file) {
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

    let books_struct = Books {
        path: folder.to_str().unwrap().to_string(),
        list: books,
    };
    save_json(&books_struct);
    Ok(books_struct)
}

pub fn save_json(books: &Books) {
    let json_file = Path::new(&books.path).join("books.json");
    let books = &books.list;
    if let Ok(mut file) = fs::File::create(json_file) {
        let serialized = serde_json::to_string(&books).unwrap();
        let _ = file.write_all(serialized.as_bytes());
    }
}
