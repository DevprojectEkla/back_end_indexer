use std::fs;
use std::io::Result;
use std::path::PathBuf;
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
