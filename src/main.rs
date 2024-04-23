extern crate serde;
extern crate xml;
// use core::time;
mod hashmaps;
mod index;
mod lexer;
mod parsers;
mod types;
mod utils;
use std::io;

use index::Index;
use utils::walk_dir;
pub fn main() {
    // let content = parse_xml_file("xml/docs.gl/gl4/glActiveShaderProgram.xhtml");
    // println!("{content:?}");
    println!("Please enter a valid path to folder to parse:");
    let mut path_input = String::new();
    match io::stdin().read_line(&mut path_input) {
        Ok(_) => println!("path selected {}:", path_input.trim()),
        Err(err) => eprintln!("Error reading path: {}\nError: {}", path_input, err),
    }
    let list_files = walk_dir(&path_input.trim());

    let mut index = Index::new(list_files);

    index.index_all();

    // thread::sleep(Duration::from_secs(1));
    // let file = File::open("xml/docs.gl/gl4/glActiveTexture.xhtml").expect("Failed to open file");
    // let file = BufReader::new(file);
    // let parser = EventReader::new(file);

    // for e in parser {
    //     let e = e.unwrap();
    //     println!("{e:?}");
    //         if let XmlEvent::Characters(text) = e {
    //             print!("{text:?}");

    // match e {
    // Ok(XmlEvent::StartElement {
    //     name, attributes, ..
    // }) => {
    //     println!("Start Element: {}", name.local_name);
    //     for attr in attributes {
    //         println!("Attribute: {}={}", attr.name.local_name, attr.value);
    //     }
    // }
    // Ok(XmlEvent::EndElement { name, .. }) => {
    //     println!("End Element: {}", name.local_name);
    // }
    // Ok(XmlEvent::Characters(data)) => {
    //     println!("Character data: {}", data);
    // }
    // Err(e) => {
    //     println!("Error: {}", e);
    //     break;
    // }
    // _ => {println!("nothing here")}

    // }
    // }
}
// }

// let all_docs = HashMap::<Path, HashMap<String, usize>>::new();
// let file_path = "docs/cours2.pdf";
// parse_pdf_file(file_path);
//     .map(|(key, value)| (key.to_string(), value))
//     .collect()
// parse_pdf_file(file_path);
