use crate::hashmaps::{index_to_json, tf_to_index, tf_to_json};
use crate::lexer::Lexer;
use crate::parsers::{check_file_type, send_to_parser};
use crate::types::{IndexDoc, PseudoHash, SliceChars, SliceContent, TermFreq};
use crate::utils::format_size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

///cette fonction est typiquement appelée avec comme argument walk_dir(path) qui retourne un Vec<String> et permet d'indexer tous les fichiers

fn index_from_string(
    path: &str,
    content: SliceChars,
    index: &mut IndexDoc,
) -> HashMap<PathBuf, PseudoHash> {
    println!("Indexing {path}...");
    let tf = index_document(&content);
    let tf_clone = tf.clone();
    let file_name = Path::new(&path)
        .file_stem()
        .expect("there should be a stem to this path")
        .to_str()
        .expect("conversion to &str must be ok")
        .trim_matches('"');
    // let file_name = path
    //     .split("/")
    //     .collect::<Vec<&str>>()
    //     .last()
    //     .expect("it should be a file name")
    //     .to_string()
    //     .split(".")
    //     .collect::<Vec<&str>>()[0]
    //     .to_string();

    tf_to_json(tf_clone, file_name);
    tf_to_index(PathBuf::from(path), tf, index);
    index.clone()
}
pub fn index_all(list_path: Vec<String>) -> () {
    let mut n = 1;
    let index = &mut IndexDoc::new();

    for path in list_path {
        let extension = check_file_type(path.as_str());
        let content = send_to_parser(path.as_str(), extension);

        match content {
            SliceContent::SliceChars(chars) => {
                index_from_string(path.as_str(), chars, index);
            }
            SliceContent::SliceBytes(value) => {
                let size = format_size(value.len());
                println!("ignoring Bytes {:?} for indexing", size)
            }
        }
        n += 1;
        // println!("content:{content:?}\nindex: {index:?} ");
    }
    index_to_json(index.clone())
}

///cette fonction est appelée par index_all pour effectivement indexer les fichiers. C'est elle qui
///réalise l'indexation d'un content &Vec<char> retourné par la fonction du parser correspondant au
///type de fichier. Elle utilise le Lexer pour générer les token des différents termes du document
///et utilise un HashMap (~un dictionnaire) pour stocker les données (terme ,fréquence) pour chaque token. Elle est aussi responsable du calcul de la fréquence des terme. TODO: faire une fonction distincte pour cette dernière opération?
fn index_document(content: &SliceChars) -> TermFreq {
    let mut tf: TermFreq = HashMap::new();
    for token in Lexer::new(content) {
        let term = token.iter().collect::<String>();
        if let Some(freq) = tf.get_mut(&term) {
            *freq += 1;
        } else {
            tf.insert(term, 1);
        }
    }
    return tf;
}
