use xml::{EventReader, EventWriter};

use crate::types::{SliceBytes, SliceChars, SliceContent};
use crate::utils::format_size;
use std::ffi::OsStr;
use std::{fs, fs::File, io::BufReader, path::Path, process::exit};

///cette fonction est appelée par la fonction index_all(). C'est elle qui redirige un fichier vers
///son parser spécifique en vu de récupérer un Vec<char> qui est en gros le contenu du fichier avant
///tokenisation c'est le retour de cette fonction qui est (passé sous la forme d'une référence) à
///index_document()
pub fn check_file_type(path: &str) -> &OsStr {
    let extension = Path::new(path).extension();
    if extension.is_some() {
        extension.expect(format!("there should be an extension to this file {path}").as_str())
    } else {
        OsStr::new("")
    }
}

pub fn send_to_parser(path: &str) -> SliceContent {
    //.extension() returns an Enum Option (None, Some)

    let extension = check_file_type(path).to_str();
    match extension {
        Some("pdf") => parse_pdf_file(path),
        Some("xml") | Some("xhtml") => parse_xml_file(path),
        Some("txt") => parse_txt_file(path),
        Some(&_) | None => parse_other_files(path),
    }
}

fn parse_other_files(path: &str) -> SliceContent {
    let content =
        fs::read(path).expect(format!("we should be able to open file {} as bytes", path).as_str());
    let size = format_size(content.len());
    let content = content.into_iter().collect();
    println!("Parsing TXT file: {path} => size:{size} ...");
    content
}
fn parse_txt_file(path: &str) -> SliceContent {
    let content = fs::read_to_string(path);
    match content {
        Ok(content) => {
            let size = format_size(content.len());
            let content = content.chars().collect();
            println!("Parsing TXT file: {path} => size:{size} ...");
            return content;
        }
        Err(err) => {
            println!("Error {}", err);
            return "".chars().collect();
        }
    }
}
pub fn parse_xml_file(path: &str) -> SliceContent {
    println!("Parsing XML file {path} ...");
    let mut content_as_string = String::new();
    let xml = BufReader::new(File::open(path.to_string()).expect("the xml file should exist"));
    let xml_event_reader = EventReader::new(xml);
    for event in xml_event_reader {
        match event {
            Ok(xml::reader::XmlEvent::Characters(string)) => {
                content_as_string.push_str(string.as_str());
            }
            _ => {
                continue;
            }
        }
    }
    content_as_string
        .chars()
        .into_iter()
        .collect::<SliceContent>()
}
fn parse_pdf_file(path: &str) -> SliceContent {
    println!("Parsing PDF file:{path} ...");
    let pdf = poppler::PopplerDocument::new_from_file(path, "").unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {path}: {err}");
        exit(1)
    });
    let n_pages = pdf.get_n_pages();
    let mut contents = String::new();
    let mut char_vec: SliceChars = Vec::new();

    for i in 0..n_pages {
        let page = pdf.get_page(i).expect(&format!(
            "get_page should workbut {i} might bee out of range"
        ));
        if let Some(content) = page.get_text() {
            char_vec = content.chars().collect::<SliceChars>();
            // for token in Lexer::new(&char_vec) {
            //     // println!("Content: {content}\nSplit Content:{char_vec:?}");
            //     println!("\nTOKEN: {token:?}");
            //     println!("=> {:?}", token.iter().collect::<String>());
            // }
            contents.push_str(content);

            // println!("{content:?}")
            let page_n = i + 1;
            println!(
                "Parsing PDF pages page {page_n}/{n_pages} => size:{size} ...",
                size = format_size(content.len())
            );
        }
    }
    if n_pages == 0 {
        return SliceContent::SliceChars(char_vec);
    }
    contents.chars().collect::<SliceContent>()
}
