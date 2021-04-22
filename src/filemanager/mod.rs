use std::path::Path;

#[derive(Clone)]
pub struct Chapter {
    pub title: String,
    pub start: i64,
    pub end: i64,
}

pub struct Book {
    pub file: String,
    pub title: String,
    pub chapters: Vec<Chapter>
}

pub fn get_book(file: &str) -> Book{

    let mut chapters = Vec::new();
    let path = Path::new(file);
    match ffmpeg::format::input(&file) {
        Ok(ictx) => {
                for chapter in ictx.chapters() {
                    let title = match chapter.metadata().get("title") {
                        Some(title) => String::from(title),
                        None => String::new(),
                    };
                    chapters.push(Chapter { title: title,
                        start: chapter.start() / 1000,
                        end: chapter.end() / 1000 });
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

    return Book {file: file.to_owned(), title: title.to_owned(),  chapters: chapters};
}