use core::time;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, metadata, DirEntry, File, Metadata, ReadDir},
    io::{Result, Write},
    path::{Path, PathBuf},
    process::exit,
    thread,
    time::Duration,
};

use walkdir::WalkDir;
///Ces types servent seulement à éviter des répétitions un peu lourde de generic types
type TermFreq = HashMap<String, usize>;
type IndexDoc = HashMap<PathBuf, TermFreq>;
type SliceContent = Vec<char>;

#[derive(Debug, Clone)]
struct Lexer<'a> {
    content: &'a [char], //lifetime 'a is needed for the whole struct
                         //when it has a field with a ref like this. the struct cannot outlive its reference
                         //the special lifetime 'static means a whole program lifetime
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) -> &'a [char] {
        //trim the blank space at the left of a token
        while self.content.len() > 0
            && (self.content[0].is_whitespace() || self.content[0].is_ascii_punctuation())
        {
            self.content = &self.content[1..];
        }
        self.content
    }
    fn trim_ascii_space(&mut self) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && self.content[0].is_ascii_punctuation() {
            self.content = &self.content[1..];
            n += 1;
            // todo!("trim_ascii_space not IMPLEMENTED")
        }
        return self.content;
    }

    fn tokenize(&mut self, n: usize) -> &'a [char] {
        //after computing the n indice we call this function to get a token from the slice
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn token_while_condition<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
        //this is non-trivial factorization of the code, basically it avoid repeating two while loop
        //but it is a bit obscure
        //keep in mind that it returns a self.tokenize(n) so it is just the same as the other if
        //condition but we add a while loop to it
        //with different condition
        //the beginning of the while loop on 'n' is always the same but the second member after '&&' changes. So we can pass this function to accept different predicate (=conditions). This predicate is a function which takes a &self.content[n] as an argument here, but where we call self.tokenizer_condition the argument is the condition on that previous &self.content argument i a closure like this: |x| x.is_alphanumeric()
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1; //
        }
        self.tokenize(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        self.trim_ascii_space();

        if self.content.len() == 0 {
            return None;
        }
        if self.content[0].is_alphabetic() {
            // the idea is simple
            // content = ['h','e','l','l','o','2','2','1','\n']
            // we iterate on this because 'h' is alphabetic and we iterate over alphanumeric
            // character (A-Z, a-z, 0-9) and we stop at /n which is not
            // alpanumeric.
            //stops at rank n = 7 which is '1' in the example
            //and return ['h','e','l','l','o','2','2','1']
            //(the \n is gone)
            return Some(self.token_while_condition(|x| x.is_alphanumeric()));
        }
        if self.content[0].is_alphanumeric() {
            //tokenize numbers 442122 or also A12323
            return Some(self.token_while_condition(|x| !x.is_ascii_whitespace()));
        }
        return Some(self.tokenize(1));
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
///cette fonction est typiquement appelée avec comme argument walk_dir(path) qui retourne un Vec<String> et permet d'indexer tous les fichiers
fn index_all(list_path: Vec<String>) -> () {
    let mut n = 1;
    for path in list_path {
        let content = check_file_type_and_send_to_parser(path.as_str());
        println!("Indexing {path}...");
        let index = index_document(&content);
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
        tf_to_json(index, file_name);
        n += 1;
        // println!("content:{content:?}\nindex: {index:?} ");
    }
}
///cette fonction est appelée par index_all pour effectivement indexer les fichiers. C'est elle qui
///réalise l'indexation d'un content &Vec<char> retourné par la fonction du parser correspondant au
///type de fichier. Elle utilise le Lexer pour générer les token des différents termes du document
///et utilise un HashMap (~un dictionnaire) pour stocker les données (terme ,fréquence) pour chaque token. Elle est aussi responsable du calcul de la fréquence des terme. TODO: faire une fonction distincte pour cette dernière opération?

fn sort_hashmap<K, V>(hashmap: HashMap<K, V>) -> Vec<(String, usize)> {
    todo!()
}

fn sort_tf_hashmap(hashmap: &TermFreq) -> Vec<(&String, &usize)> {
    let mut stats = hashmap.into_iter().collect::<Vec<_>>(); // returns [("key1", value1),...,]
    stats.sort_by_key(|(_, freq)| *freq);
    //unstable_by_key(|x| (*x).1);
    stats.reverse();
    stats
}

fn index_document(content: &SliceContent) -> TermFreq {
    let mut tf: TermFreq = HashMap::new();
    for token in Lexer::new(content) {
        let term = token.iter().collect::<String>();
        if let Some(freq) = tf.get_mut(&term) {
            *freq += 1;
        } else {
            tf.insert(term, 1);
        }
        // println!("Content: {content:?}\n");
        // println!("\nTOKEN: {token:?}");
        // println!("=> {:?}", token.iter().collect::<String>());
        // hashmap.insert(token.iter().collect::<String>(), token.len());
    }
    // let mut stats = tf.iter().collect::<Vec<_>>();
    // stats.sort_by_key(|(_, freq)| *freq);
    // stats.reverse();

    // for (t, f) in stats.iter().take(10) {
    //     println!("term {t} => {f} times")
    // }
    // let sorted = sort_hashmap(tf);
    // println!("{:?}", sorted.iter().take(10));
    // thread::sleep(Duration::from_secs(3));
    // sorted
    //     .into_iter()
    //     .map(|(key, value)| (key.to_string(), value))
    //     .collect()
    return tf;
    // println!("{:?}", tf);
    // let sorted = sort_hashmap(tf);
    // sorted
    //     .into_iter()
    //     .map(|(key, value)| (key.to_string(), value))
    //     .collect()

    // todo!("not implemented yet")
}
///cette fonction est appelée par la fonction index_all(). C'est elle qui redirige un fichier vers
///son parser spécifique en vu de récupérer un Vec<char> qui est en gros le contenu du fichier avant
///tokenisation c'est le retour de cette fonction qui est (passé sous la forme d'une référence) à
///index_document()
fn check_file_type_and_send_to_parser(path: &str) -> SliceContent {
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
/// this is the function that iterate on each directory to retrieve all files of the whole tree
/// starting from path which should logically but not necessarily be a path to a directory. On each
/// files it calls the function that will check the extension and send the path file to the parser
fn walk_dir(path: &str) -> Vec<String> {
    let mut i = 1;
    let mut n_files = 1;
    let mut list_path: Vec<String> = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry
            .expect("WalkDir doit bien donner une valeur pour entry")
            .clone();
        if entry.path().is_file() {
            let path_to_file = entry.path().to_str().expect("should match");
            list_path.push(path_to_file.to_string());

            // let content =
            //     check_file_type_and_send_to_parser(entry.path().to_str().expect("should match"));
            // let index = index_document(&content);

            n_files += 1;
            i += 1;
            // let pause = time::Duration::from_secs(1);
            // thread::sleep(pause);
        }
    }
    println!("number of file:  {n_files} ");
    list_path
}

fn check_is_dir(path_dir: &Path) -> Result<bool> {
    let meta = metadata(path_dir)?.file_type().is_dir();

    Ok(meta)
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
    println!("file:{path} => size:{size}");

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

fn format_size(size: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;
    const GB: f64 = 1024.0 * MB;
    const TB: f64 = 1024.0 * GB;

    if size < KB as usize {
        format!("{} B", size)
    } else if size < MB as usize {
        format!("{:.2} KB", size as f64 / KB)
    } else if size < GB as usize {
        format!("{:.2} MB", size as f64 / MB)
    } else if size < TB as usize {
        format!("{:.2} GB", size as f64 / GB)
    } else {
        format!("{:.2} TB", size as f64 / TB)
    }
}
fn tf_to_index(path: PathBuf, tf: TermFreq) -> IndexDoc {
    let mut index_doc = IndexDoc::new();
    index_doc.insert(path, tf);
    index_doc
}
fn index_to_json(index: IndexDoc) {
    todo!()
}
fn tf_to_json(tf: TermFreq, file_stem: &str) {
    hashmap_to_json(&tf, file_stem, sort_tf_hashmap);
}
fn hashmap_to_json<K, V, F>(hashmap: &HashMap<K, V>, name: &str, mut sort_function: F)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
    F: FnMut(&HashMap<K, V>) -> Vec<(&String, &usize)>,
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
            .map(|(k, v)| format!(r#""{:?}":"{:?}""#, k, v))
            .collect::<Vec<_>>()
            .join(",\n")
    );

    let mut json = File::create(format!("data/_index-{name}.json"))
        .expect("we should be able to create a file on the system");
    json.write_all(json_format.as_bytes())
        .expect("could write to a simple file")
}

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
