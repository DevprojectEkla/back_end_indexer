// use core::time;
mod hashmaps;
mod index;
mod lexer;
mod parsers;
mod types;
mod utils;
use index::index_all;
use utils::walk_dir;

// /// fonction générale de tri des Hashmaps avec possibilité de choix d'une méthode de tri
//fn sort_hashmap<'a, 'b, K, V, F>(hashmap: &'a HashMap<K, V>, mut sort_method: F) -> Vec<(&K, &V)>
//where
//    K: std::fmt::Debug,
//    V: std::fmt::Debug,
//    F: FnMut(Vec<(&'a K, &'a V)>) -> Vec<(&'a K, &'a V)>,
//{
//    let stats = hashmap.into_iter().collect::<Vec<_>>(); // returns [("key1", value1),...,]
//    let stats = sort_method(stats.clone());
//    //unstable_by_key(|x| (*x).1);
//    stats
//}
//fn sort_index<'a>(index: &'a IndexDoc) -> Vec<(&'a PathBuf, &'a TermFreq)> {
//    let sort_method = |mut stats: Vec<(&'a PathBuf, &'a TermFreq)>| {
//        stats.sort_by_key(|(k, _)| *k);
//        stats
//    };
//    sort_hashmap(index, sort_method)
//}

//utiliser une fonction generale pour le tri ne permet pas de factoriser le code cf. plus haut avec
//sort_hashmap qui voulait prendre en argument n'importe quel Hashmap et invoquer dessus une
//methode de tri, on se retrouve a devoir reecrire ce qui suit

fn main() {
    let list_files = walk_dir("docs/");
    // thread::sleep(Duration::from_secs(1));
    index_all(list_files);

    // let all_docs = HashMap::<Path, HashMap<String, usize>>::new();
    // let file_path = "docs/cours2.pdf";
    // parse_pdf_file(file_path);
    //     .map(|(key, value)| (key.to_string(), value))
    //     .collect()
    // parse_pdf_file(file_path);
}
