extern crate serde;
extern crate xml;
// use core::time;
pub mod hashmaps;
pub mod index;
pub mod lexer;
pub mod parsers;
pub mod types;
pub mod utils;
use index::Index;
use utils::walk_dir;

pub fn index_folder(folder: &str) {
    // let content = parse_xml_file("xml/docs.gl/gl4/glActiveShaderProgram.xhtml");
    // println!("{content:?}");
    let list_files = walk_dir(folder);

    let mut index = Index::new(list_files.clone());

    index.index_all();
}
