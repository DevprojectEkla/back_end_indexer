use crate::types::{IndexDoc, TermFreq};
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
    index_doc.insert(path, tf);
    index_doc
}
pub fn index_to_json(index: IndexDoc) {
    hashmap_to_json(&index, "index", sort_index)
}
pub fn tf_to_json(tf: TermFreq, file_stem: &str) {
    hashmap_to_json(&tf, file_stem, sort_tf_hashmap);
}
fn hashmap_to_json<K, V, F>(hashmap: &HashMap<K, V>, name: &str, mut sort_function: F)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
    F: FnMut(&HashMap<K, V>) -> Vec<(&K, &V)>,
{
    // let mut data = <Vec<_>>::new();
    let data = sort_function(hashmap);
    // for (key, value) in hashmap {
    //     data.push(format!(r#""{:?}":"{:?}",\n"#, key, value));
    // }
    let json_format = format!(
        r#"{{{}}}
"#,
        data.iter()
            .map(|(k, v)| format!(r#"{:?}:{:?}"#, *k, v))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    let mut json = File::create(format!("data/_index-{name}.json"))
        .expect("we should be able to create a file on the system");
    json.write_all(json_format.as_bytes())
        .expect("could write to a simple file")
}

fn hashmap_into_vec<K, V>(hashmap: &HashMap<K, V>) -> Vec<(&K, &V)> {
    hashmap.into_iter().collect::<Vec<_>>()
}

fn sort_index(index: &IndexDoc) -> Vec<(&PathBuf, &TermFreq)> {
    let mut stats = hashmap_into_vec(index);
    stats.sort_by_key(|(k, _)| *k);
    //unstable_by_key(|x| (*x).1);
    stats
}

pub fn sort_tf_hashmap(hashmap: &TermFreq) -> Vec<(&String, &usize)> {
    let mut stats = hashmap_into_vec(hashmap); // returns [("key1", value1),...,]
    stats.sort_by_key(|(_, freq)| *freq);
    //unstable_by_key(|x| (*x).1);
    stats.reverse();
    stats
}
