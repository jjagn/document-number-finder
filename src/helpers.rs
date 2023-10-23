use crate::{DirEntry, Document, Path};

use std::io::{stdin, stdout, Read, Write};


pub fn pause() -> bool {
    let mut stdin = stdin();
    let mut stdout = stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
    // println!("{}", pressed);
    false
}

pub fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}
pub fn doc_path_without_top_level_dirs(doc: Document) -> String {
    let doc_ancestors: Vec<&Path> = doc.path.ancestors().collect();
    let mut doc_ancestors_as_string: Vec<&str> = Vec::new();
    for path in &doc_ancestors {
        doc_ancestors_as_string.push(path.to_str().unwrap());
    }

    let path_without_initial = doc.path.to_str().unwrap().replace(
        doc_ancestors_as_string[doc_ancestors_as_string.len() - 3],
        "",
    );
    path_without_initial
}
