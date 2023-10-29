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
    content: &'a [char], //lifetime 'a is needed for the whole struct
                         //when it has a field with a ref like this. the struct cannot outlive its reference
                         //the special lifetime 'static means a whole program lifetime
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) -> &'a [char] {
        //trim the blank space at the left of a token
        while self.content.len() > 0
            && (self.content[0].is_whitespace() || self.content[0].is_ascii_punctuation())
        {
            self.content = &self.content[1..];
        }
        self.content
    }
    fn trim_ascii_space(&mut self) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && self.content[0].is_ascii_punctuation() {
            self.content = &self.content[1..];
            n += 1;
            // todo!("trim_ascii_space not IMPLEMENTED")
        }
        return self.content;
    }

    fn tokenize(&mut self, n: usize) -> &'a [char] {
        //after computing the n indice we call this function to get a token from the slice
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn token_while_condition<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
        //this is non-trivial factorization of the code, basically it avoid repeating two while loop
        //but it is a bit obscure
        //keep in mind that it returns a self.tokenize(n) so it is just the same as the other if
        //condition but we add a while loop to it
        //with different condition
        //the beginning of the while loop on 'n' is always the same but the second member after '&&' changes. So we can pass this function to accept different predicate (=conditions). This predicate is a function which takes a &self.content[n] as an argument here, but where we call self.tokenizer_condition the argument is the condition on that previous &self.content argument i a closure like this: |x| x.is_alphanumeric()
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1; //
        }
        self.tokenize(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        self.trim_ascii_space();

        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_alphabetic() {
            // the idea is simple
            // content = ['h','e','l','l','o','2','2','1','\n']
            // we iterate on this because 'h' is alphabetic and we iterate over alphanumeric
            // character (A-Z, a-z, 0-9) and we stop at /n which is not
            // alpanumeric.
            //stops at rank n = 7 which is '1' in the example
            //and return ['h','e','l','l','o','2','2','1']
            //(the \n is gone)
            return Some(self.token_while_condition(|x| x.is_alphanumeric()));
        }
        if self.content[0].is_alphanumeric() {
            //tokenize numbers 442122 or also A12323
            return Some(self.token_while_condition(|x| !x.is_ascii_whitespace()));
        }
        return Some(self.tokenize(1));
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
fn index_document(content: &Vec<char>) -> HashMap<String, usize> {
    let mut hashmap: HashMap<String, usize> = HashMap::new();
    for token in Lexer::new(content) {
        // println!("Content: {content:?}\n");
        println!("\nTOKEN: {token:?}");
        println!("=> {:?}", token.iter().collect::<String>());
        hashmap.insert(token.iter().collect::<String>(), token.len());
    }
    println!("{:?}", hashmap);
    hashmap

    // todo!("not implemented yet")
}

fn check_file_type_and_send_to_parser(path: &str) -> Vec<char> {
    let extension = Path::new(path).extension().expect("should be an extension");
    //.extension() returns an Enum Option (None, Some)
    match extension.to_str() {
        Some("pdf") => parse_pdf_file(path),
        Some("xml") => parse_xml_file(path),
        Some(&_) => parse_other_files(path),
        None => parse_no_extension(path),
    }
}
/// this is the function that iterate on each directory to retrieve all files of the whole tree
/// starting from path which should logically but not necessarily be a path to a directory. On each
/// files it calls the function that will check the extension and send the path file to the parser
fn walk_dir(path: &str) -> Result<()> {
    let mut i = 1;
    let mut n_files = 1;
    for entry in WalkDir::new(path) {
        i += 1;
        let entry = entry?.clone();
        if entry.path().is_file() {
            let content =
                check_file_type_and_send_to_parser(entry.path().to_str().expect("should match"));
            let index = index_document(&content);

            println!("number of iteration: {i}, number of file:  {n_files} content:{content:?}\nindex: {index:?} ");
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

// fn check_is_pdf(path: &str) -> bool {
//     let extension = Some(OsStr::new("pdf"));
//     if Path::new(path).extension() == extension {
//         return true;
//     } else {
//         false
//     }
// }
fn parse_no_extension(path: &str) -> Vec<char> {
    todo!("no extension parser to implement to parse {path}");
}
fn parse_other_files(path: &str) -> Vec<char> {
    todo!("other parser to implement to parse {path}");
}
fn parse_xml_file(path: &str) -> Vec<char> {
    todo!("xml parser to implement to parse {path}");
}
fn parse_pdf_file(path: &str) -> Vec<char> {
    let pdf = poppler::PopplerDocument::new_from_file(path, "").unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {path}: {err}");
        exit(1)
    });
    let n_pages = pdf.get_n_pages();
    let mut contents = String::new();
    let mut char_vec: Vec<char> = Vec::new();

    for i in 0..n_pages {
        let page = pdf.get_page(i).expect(&format!(
            "get_page should workbut {i} might bee out of range"
        ));
        if let Some(content) = page.get_text() {
            char_vec = content.chars().collect::<Vec<char>>();
            // for token in Lexer::new(&char_vec) {
            //     // println!("Content: {content}\nSplit Content:{char_vec:?}");
            //     println!("\nTOKEN: {token:?}");
            //     println!("=> {:?}", token.iter().collect::<String>());
            // }
            contents.push_str(content);
            // println!("{content:?}")
            let page_n = i + 1;
            println!(
                "contents: {contents}, file:{path}, page {page_n}/{n_pages} => size:{size}",
                size = format_size(content.len())
            );
            return char_vec;
        } else {
            return char_vec;
        }
    }
    if n_pages == 0 {
        return char_vec;
    }
    char_vec
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
    let _ = walk_dir("docs/numbers.pdf");
    // let all_docs = HashMap::<Path, HashMap<String, usize>>::new();
    // let file_path = "docs/cours2.pdf";
    // parse_pdf_file(file_path);
}
