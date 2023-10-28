use core::time;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, metadata, DirEntry, File, Metadata, ReadDir},
    io::Result,
    path::Path,
    process::exit,
    thread,
    time::Duration,
};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) -> &'a [char] {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
        self.content
    }
    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_alphabetic() {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_alphanumeric() {
                n += 1;
            }
            let token = &self.content[0..n];
            self.content = &self.content[n..];
            return Some(token);
        }
        let token = &self.content[0..1];
        self.content = &self.content[1..];
        return Some(token);
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
fn index_document(content: &str) -> HashMap<String, usize> {
    todo!("not implemented yet")
}

fn walk_dir(path: &str) -> Result<()> {
    let mut i = 1;
    let mut n_files = 1;
    for entry in WalkDir::new(path) {
        i += 1;
        let entry = entry?.clone();
        if entry.path().is_file()
            && check_is_pdf(
                entry
                    .path()
                    .to_str()
                    .expect("should find a path for that entry"),
            )
        {
            parse_pdf_file(entry.path().to_str().expect("should match"));
            println!("number of iteration: {i}, number of pdf file:  {n_files}");
            n_files += 1;
            let pause = time::Duration::from_secs(1);
            thread::sleep(pause);
        }
    }
    Ok(())
}
fn check_is_dir(path_dir: &Path) -> Result<bool> {
    let meta = metadata(path_dir)?.file_type().is_dir();

    Ok(meta)
}

fn check_is_pdf(path: &str) -> bool {
    let extension = Some(OsStr::new("pdf"));
    if Path::new(path).extension() == extension {
        return true;
    } else {
        false
    }
}

fn parse_pdf_file(path: &str) {
    if check_is_pdf(path) {
        let pdf = poppler::PopplerDocument::new_from_file(path, "").unwrap_or_else(|err| {
            eprintln!("ERROR: could not read file {path}: {err}");
            exit(1)
        });
        let n_pages = pdf.get_n_pages();
        let mut contents = String::new();

        for i in 0..n_pages {
            let page = pdf.get_page(i).expect(&format!(
                "get_page should workbut {i} might bee out of range"
            ));
            if let Some(content) = page.get_text() {
                let char_vec = &content.chars().collect::<Vec<char>>();
                for token in Lexer::new(&char_vec) {
                    println!("{:?}", token.iter().collect::<String>());
                }
                contents.push_str(content);
                // println!("{content:?}")
                let page_n = i + 1;
                println!(
                    "file:{path}, page {page_n}/{n_pages} => size:{size}",
                    size = format_size(content.len())
                );
            }
        }
    }
}

fn format_size(size: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;
    const GB: f64 = 1024.0 * MB;
    const TB: f64 = 1024.0 * GB;

    if size < KB as usize {
        format!("{} B", size)
    } else if size < MB as usize {
        format!("{:.2} KB", size as f64 / KB)
    } else if size < GB as usize {
        format!("{:.2} MB", size as f64 / MB)
    } else if size < TB as usize {
        format!("{:.2} GB", size as f64 / GB)
    } else {
        format!("{:.2} TB", size as f64 / TB)
    }
}

fn main() {
    let _ = walk_dir("docs/");
    // let all_docs = HashMap::<Path, HashMap<String, usize>>::new();
    // let file_path = "docs/cours2.pdf";
    // parse_pdf_file(file_path);
}
