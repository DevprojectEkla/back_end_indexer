use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

//extern crate to browse an entire tree of directories
use walkdir::WalkDir;

enum Test {
    Test1,
    Test2,
    Test3,
}

enum Option {
    Some(String),
    None,
}

enum Result {
    Ok(),
    Err(),
}

/// this is the function that iterate on each directory to retrieve all files of the whole tree
/// starting from path which should logically but not necessarily be a path to a directory. On each
/// files it calls the function that will check the extension and send the path file to the parser
pub fn walk_dir(path: &str) -> Vec<String> {
    let mut i = 1;
    let mut n_files = 1;
    let mut list_path: Vec<String> = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.expect("WalkDir doit bien donner une valeur pour entry");
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
    println!("number of file:  {} ", n_files);
    list_path
}

// fn check_is_dir(path_dir: &Path) -> Result<bool> {
//     let meta = metadata(path_dir)?.file_type().is_dir();

//     Ok(meta)
// }

pub fn generate_uid() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Judgment Day has arrived")
        .as_micros();
    let random_number: u64 = rand::random();
    let id = format!("{}-{}", timestamp, random_number);
    id
}

pub fn format_size(size: usize) -> String {
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

pub fn trim_quotes_from_path(path: PathBuf) -> PathBuf {
    let path_str = path.to_str().unwrap_or_default(); // Convert PathBuf to a string
    let trimmed_path_str = path_str.trim_matches('"'); // Remove single quotes
    PathBuf::from(trimmed_path_str) // Convert the trimmed string back to a PathBuf
}

/*
//generated by chatGPT, interesting but to verbose

fn process_entries_in_directory<F>(path: &str, process_entry: F) -> Result<()>
where
    F: Fn(fs::DirEntry) -> Result<()>,
{
    let dir_tree = fs::read_dir(path)?;

    for entry in dir_tree {
        let dir_entry = entry?;
        process_entry(dir_entry)?;
    }

    Ok(())
}

fn process_entry(dir_entry: fs::DirEntry) -> Result<()> {
    let metadata = dir_entry.metadata()?;
    let path_dir = dir_entry.path();

    if metadata.is_dir() {
        println!("directory");
        println!("Path: {path_dir:?}"); //, Metadata: {:?}", path_dir, metadata);
        walk_dir(&path_dir.to_str().unwrap())?;
    } else {
        println!("Path: {path_dir:?}"); // Metadata: {:?}", path_dir, metadata);
    }

    Ok(())
}
fn walk_dir(path: &str) -> Result<()> {
    process_entries_in_directory(path, |dir_entry| process_entry(dir_entry))?;

    Ok(())
}
fn check_is_dir(path_dir: &Path) -> Result<bool> {
    let meta = metadata(path_dir)?.file_type().is_dir();

    Ok(meta)
}

*/
