use crate::types::SliceContent;
use crate::utils::format_size;
use std::{fs, path::Path, process::exit};

///cette fonction est appelée par la fonction index_all(). C'est elle qui redirige un fichier vers
///son parser spécifique en vu de récupérer un Vec<char> qui est en gros le contenu du fichier avant
///tokenisation c'est le retour de cette fonction qui est (passé sous la forme d'une référence) à
///index_document()
pub fn check_file_type_and_send_to_parser(path: &str) -> SliceContent {
    let extension = Path::new(path).extension().expect("should be an extension");
    //.extension() returns an Enum Option (None, Some)
    match extension.to_str() {
        Some("pdf") => parse_pdf_file(path),
        Some("xml") => parse_xml_file(path),
        Some("txt") => parse_txt_file(path),
        Some(&_) => parse_other_files(path),
        None => parse_no_extension(path),
    }
}

fn parse_no_extension(path: &str) -> SliceContent {
    todo!("no extension parser to implement to parse {path}");
}
fn parse_other_files(path: &str) -> SliceContent {
    todo!("other parser to implement to parse {path}");
}
fn parse_txt_file(path: &str) -> SliceContent {
    let content = fs::read_to_string(path).expect("we should be able to open a .txt file");
    let size = format_size(content.len());
    let content = content.chars().collect();
    println!("Parsing TXT file: {path} => size:{size} ...");

    return content;
}
fn parse_xml_file(path: &str) -> SliceContent {
    todo!("xml parser to implement to parse {path}");
}
fn parse_pdf_file(path: &str) -> SliceContent {
    println!("Parsing PDF file:{path} ...");
    let pdf = poppler::PopplerDocument::new_from_file(path, "").unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {path}: {err}");
        exit(1)
    });
    let n_pages = pdf.get_n_pages();
    let mut contents = String::new();
    let mut char_vec: SliceContent = Vec::new();

    for i in 0..n_pages {
        let page = pdf.get_page(i).expect(&format!(
            "get_page should workbut {i} might bee out of range"
        ));
        if let Some(content) = page.get_text() {
            char_vec = content.chars().collect::<SliceContent>();
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
        return char_vec;
    }
    contents.chars().collect::<SliceContent>()
}
