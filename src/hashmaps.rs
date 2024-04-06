use crate::types::{IndexDoc, PseudoHash, TermFreq};
use serde;
use serde_json;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
struct Index<K, V> {
    hashmap: HashMap<K, V>,
}
impl<K, V> Index<K, V> {
    fn new(&mut self) -> Self {
        todo!()
    }
}
pub fn tf_to_index(path: PathBuf, tf: TermFreq, index_doc: &mut IndexDoc) -> &mut IndexDoc {
    // let sorted = sort_tf_for_index(tf);
    // println!("{:?}", sorted);
    index_doc.insert(path, tf);
    println!("{:?}", index_doc);
    index_doc
}
pub fn index_to_json(index: IndexDoc) {
    hashmap_to_json(index, "index", sort_index)
}
pub fn tf_to_json(tf: TermFreq, file_stem: &str) {
    hashmap_to_json(tf, file_stem, sort_tf_for_index);
}

fn vec_to_json<K, V>(vec: Vec<(K, V)>) -> String
where
    K: std::fmt::Debug + serde::Serialize,
    V: std::fmt::Debug + serde::Serialize,
{
    let json_format = format!(
        r#"{{{:?}}}"#,
        vec.iter()
            .map(|(k, v)| format!(r#"{:?}:{:?}"#, *k, v))
            .collect::<Vec<_>>()
            .join(",\n")
    );
    json_format
}

fn hashmap_to_json<K, V, F>(hashmap: HashMap<K, V>, name: &str, mut sort_function: F)
where
    K: std::fmt::Debug + serde::Serialize,
    V: std::fmt::Debug + serde::Serialize,
    F: FnMut(HashMap<K, V>) -> Vec<(K, V)>,
{
    // let mut data = <Vec<_>>::new();
    // let data = sort_function(hashmap);
    let data = hashmap;

    // let json_format = format!(
    //     r#"{{{}}}
    // "#,
    //     data.iter()
    //         .map(|(k, v)| format!(r#"{:?}:{:?}"#, *k, v))
    //         .collect::<Vec<_>>()
    //         .join(",\n")
    // );
    let json_format = serde_json::to_string(&data)
        .expect(format!("serialization with serde should work for {name:?}").as_str());
    let mut json = File::create(format!("data/_index-{name}.json"))
        .expect("we should be able to create a file on the system");
    json.write_all(json_format.as_bytes())
        .expect("could write to a simple file")
}

fn vec_into_hashmap<K: std::hash::Hash + std::cmp::Eq, V>(vec: Vec<(K, V)>) -> HashMap<K, V> {
    vec.into_iter().collect::<HashMap<K, V>>()
}
fn hashmap_into_vec<K, V>(hashmap: HashMap<K, V>) -> Vec<(K, V)> {
    hashmap.into_iter().collect::<Vec<_>>()
}

fn sort_index(index: IndexDoc) -> Vec<(PathBuf, TermFreq)> {
    let mut stats = hashmap_into_vec(index);
    stats.sort_by_key(|(a, _)| a.clone());
    //unstable_by_key(|x| (*x).1);
    stats
}
fn sort_tf_for_index(hashmap: TermFreq) -> PseudoHash {
    let mut stats = hashmap_into_vec(hashmap);
    stats.sort_by_key(|(_, freq)| *freq as usize);
    stats.reverse();
    stats
}

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
