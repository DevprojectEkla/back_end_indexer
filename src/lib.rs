extern crate serde;
extern crate xml;
// use core::time;
pub mod hashmaps;
pub mod index;
pub mod lexer;
pub mod parsers;
pub mod types;
pub mod utils;
use index::index_all;
use utils::walk_dir;

pub fn index_folder(folder: &str) {
    // let content = parse_xml_file("xml/docs.gl/gl4/glActiveShaderProgram.xhtml");
    // println!("{content:?}");
    let list_files = walk_dir(folder);
    index_all(list_files);
}
