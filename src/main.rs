use std::{
    fs::{self, metadata, DirEntry, File, Metadata, ReadDir},
    io::Result,
    path::Path,
    process::exit,
};

fn walk_dir(path: &str) -> Result<()> {
    let dir_tree = fs::read_dir(path)?;
    for entry in dir_tree {
        if let Some(dir) = entry.ok() {
            let metadata = dir.metadata();
            let path_dir = dir.path();
            if check_is_dir(path_dir.as_path())
                .ok()
                .expect("check_dir should work")
            {
                println!("directory")
            }
            println!("{path_dir:?}, metadata:{metadata:?}")
        }
    }

    Ok(())
}

fn check_is_dir(path_dir: &Path) -> Result<bool> {
    let meta = metadata(path_dir)?.file_type().is_dir();

    Ok(meta)
}
fn parse_pdf_file(path: &str) {
    let pdf = poppler::PopplerDocument::new_from_file(path, "").unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {path}: {err}");
        exit(1)
    });
    let n_pages = pdf.get_n_pages();
    let mut contents = String::new();

    for i in 0..n_pages {
        let page = pdf.get_page(i).expect(&format!(
            "get_page should workbut {i} might bee out of range"
        ));
        if let Some(content) = page.get_text() {
            contents.push_str(content);
            println!("{content:?}")
        }
    }
}
fn main() {
    let _ = walk_dir(".");
    let file_path = "docs/cours2.pdf";
    parse_pdf_file(file_path);
}
