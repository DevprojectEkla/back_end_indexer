use crate::hashmaps::{index_to_json, tf_to_index, tf_to_json};
use crate::lexer::Lexer;
use crate::parsers::{check_file_type, send_to_parser};
use crate::types::{IndexDoc, PseudoHash, SliceChars, SliceContent, TermFreq, TfIdF};
use crate::utils::format_size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

///cette fonction est typiquement appelée avec comme argument walk_dir(path) qui retourne un Vec<String> et permet d'indexer tous les fichiers
#[derive(Clone, Debug)]
pub struct Index {
    list_path: Vec<String>,
    index: IndexDoc,
    number_of_documents: usize,
    tf: usize, // tf/idf is the tyical weight calculation for search_engine
    idf: usize,
}
impl Index {
    pub fn new(list_path: Vec<String>) -> Self {
        let index = IndexDoc::new();
        let tf = 0;
        let idf = 0;
        let number_of_documents = list_path.len();

        Self {
            list_path,
            number_of_documents,
            index,
            tf,
            idf,
        }
    }

    fn index_from_string(&mut self, path: &str, content: SliceChars) -> HashMap<PathBuf, TermFreq> {
        println!("Indexing {path}...");
        let tf = self.index_document(&content);
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
        tf_to_index(PathBuf::from(path), tf, &mut self.index);
        self.index.clone()
    }
    pub fn index_all(&mut self) -> () {
        let mut n = 1;

        for path in self.list_path.clone() {
            let content = send_to_parser(path.as_str());

            match content {
                SliceContent::SliceChars(chars) => {
                    self.index_from_string(path.as_str(), chars);
                }
                SliceContent::SliceBytes(value) => {
                    let size = format_size(value.len());
                    println!("ignoring Bytes {:?} for indexing", size)
                }
            }
            n += 1;
            // println!("content:{content:?}\nindex: {index:?} ");
        }
        index_to_json(self.index.clone())
    }

    ///cette fonction est appelée par index_all pour effectivement indexer les fichiers. C'est elle qui
    ///réalise l'indexation d'un content &Vec<char> retourné par la fonction du parser correspondant au
    ///type de fichier. Elle utilise le Lexer pour générer les token des différents termes du document
    ///et utilise un HashMap (~un dictionnaire) pour stocker les données (terme ,fréquence) pour chaque token. Elle est aussi responsable du calcul de la fréquence des terme. TODO: faire une fonction distincte pour cette dernière opération?
    fn index_document(&self, content: &SliceChars) -> TermFreq {
        let mut tf: TermFreq = HashMap::new();
        let lexer = Lexer::new(content);

        let number_of_terms = lexer.clone().into_iter().count() as f32;

        for token in lexer {
            let term = token.iter().collect::<String>();
            if let Some(freq) = tf.get_mut(&term) {
                *freq = *freq + 1f32 / number_of_terms as f32;
            } else {
                tf.insert(term, 1f32 / number_of_terms);
            }
        }

        println!("number_of_terms:{}", number_of_terms);
        return tf;
    }

    pub fn idf_calculation(&self, search_term: &str) -> TfIdF {
        //the formula is log(N/1+nt) number of docs / number of docs containing t
        let mut nt: f32 = 0.0;
        let mut doc_list = Vec::<&PathBuf>::new();
        let mut tf_idf = TfIdF::new();
        let mut tf: f32;
        for (doc, tf_hashmap) in &self.index {
            if tf_hashmap.contains_key(search_term) {
                nt += 1.0;
                doc_list.push(doc);
            }
        }
        let idf = (self.number_of_documents as f32 / nt).ln();

        for doc in doc_list {
            let hash = self
                .index
                .get(doc)
                .expect("no hashmap corresponding to this doc path");

            tf = *hash.get(search_term).expect("there should be a tf value");
            let weight = tf * idf;
            tf_idf.insert(doc.clone(), weight);
        }

        return tf_idf;
    }
}
