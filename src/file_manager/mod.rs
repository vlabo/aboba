use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use mp4ameta::Tag;
use serde_json::{Value, json};

#[derive(Clone, Default, Debug)]
pub struct Chapter {
    pub title: String,
    pub start: i64,
    pub end: i64,
    pub duration: i64,
}

#[derive(Clone, Default)]
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
    let mut chapters = Vec::new();
    let path = Path::new(file);
    let mut title: String = "Unknown".to_owned();

    let tag = Tag::read_from_path(file);

    if let Ok(tag) = tag {
    	for chapter in tag.chapters() {
    		chapters.push(Chapter {
    			title: chapter.title.to_string(),
    			start: chapter.start.as_secs() as i64,
    			end: chapter.start.as_secs() as i64 + chapter.duration.as_secs() as i64,
    			duration: chapter.duration.as_secs() as i64,
    		});
    	}

        if chapters.is_empty() {
            let duration = tag.duration().unwrap().as_secs() as i64;
            chapters.push(Chapter {
                title: "Chapter".to_string(),
                start: 0,
                end: duration,
                duration: duration,
            });
        }

        if let Some(tag_title) = tag.title() {
            title = tag_title.to_string();
        } else if let Some(file_stem) = path.file_stem() {
            if let Some(file_name) = file_stem.to_str() {
                title = file_name.to_owned();
            }
        }
    }
    
    return Book {
        file: file.to_owned(),
        title: title,
        chapters: chapters,
        time: 0,
    };
}

pub fn init_dir(folder: &Path) -> io::Result<Books> {
    let mut json_books = Value::Null;

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
                        let mut book = get_book(&file_str);
                    
                        if let Value::Array(ref array) = json_books {
                            for json_book in array {
                                if json_book["file"].eq(&book.file) {
                                    if let Value::Number(time) = &json_book["time"] {
                                        book.time = time.as_i64().unwrap_or(0);
                                    }
                                }
                            }
                        }
                        
                        books.push(book);
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
    let mut values: Vec<Value> = Vec::new();

    for book in &books.list {
        values.push(json!({
            "file": book.file,
            "time": book.time,
        }));
    }
    let json_books = Value::Array(values);

    if let Ok(mut file) = fs::File::create(json_file) {
        let _ = file.write_all(json_books.to_string().as_bytes());
    }
}
