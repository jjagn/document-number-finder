use std::{
    collections::HashMap,
    io::{self, prelude::*},
    path::PathBuf,
};
use terminal_link::Link;

use colored::*;

use regex::Regex;
use terminal_menu::{button, label, menu, mut_menu, run};
use walkdir::{DirEntry, WalkDir};

extern crate walkdir;

#[derive(Clone)]
struct Document {
    index: i32,
    path: PathBuf,
}

#[derive(Clone)]
struct DocumentIndex(i32, Vec<Document>);

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn dummy_to_run_menu() -> Result<PathBuf, String> {
    let development_path = "H:\\Development";
    let projects = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    let mut file_pick_menu_items = Vec::new();
    file_pick_menu_items.push(label("Select folder"));

    for item in projects {
        let dir_name = item.unwrap();
        let dir_name2 = dir_name.path().file_name().unwrap().to_str().unwrap();
        // println!("{}", item?.path().file_name().unwrap().to_str().unwrap());
        file_pick_menu_items.push(button(dir_name2));
    }

    let new_menu = menu(file_pick_menu_items);

    run(&new_menu);
    let mm = mut_menu(&new_menu);
    let dir_name = mm.selected_item_name();

    let projects2 = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    for item in projects2 {
        let dir_name_check_option = item.unwrap();
        let dir_name_check = dir_name_check_option
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        if dir_name_check == dir_name {
            return Ok(dir_name_check_option.into_path());
        }
    }
    Err("path not found".to_string())
}

fn main() -> io::Result<()> {
    let menu = menu(vec![
        label("select document code:"),
        button("1.5.1 -> Packaging Selection"),
        button("2.1 -> Usability Assessment"),
        button("3.2.1 -> Theoretical Review"),
        button("3.2.2 -> Peer Review"),
        button("3.3 -> Design Equivalence Claim"),
        button("3.4 -> Test Report"),
        button("3.5.3 -> Feedback Summary"),
        button("3.7.1 -> Biocompatibility Claim"),
        button("3.7.2 -> Cleaning Sterilisation Claim"),
    ]);

    run(&menu);
    let mm = mut_menu(&menu);
    let user_input = mm
        .selected_item_name()
        .split_whitespace()
        .next()
        .unwrap_or("");

    let mut re2 = Regex::new("").unwrap();

    match user_input.trim() {
        "1.5.1" => re2 = Regex::new(r"PackagingSelection\d{1,}").unwrap(),
        "2.1" => re2 = Regex::new(r"UsabilityAssess\d{1,}").unwrap(),
        "3.2.1" => re2 = Regex::new(r"TheoreticalReview\d{1,}").unwrap(),
        "3.2.2" => re2 = Regex::new(r"PeerReview\d{1,}").unwrap(),
        "3.3" => re2 = Regex::new(r"DesignEquivClaim\d{1,}").unwrap(),
        "3.4" => re2 = Regex::new(r"TestReport\d{1,}").unwrap(),
        "3.5.3" => re2 = Regex::new(r"FeedbackSummary\d{1,}").unwrap(),
        "3.7.1" => re2 = Regex::new(r"BiocompClaim\d{1,}").unwrap(),
        "3.7.2" => re2 = Regex::new(r"CleaningSteriClaim\d{1,}").unwrap(),
        // "h" | "H" => {
        //     println!("1.5.1 -> Packaging Selection");
        //     println!("2.1 -> Usability Assessment");
        //     println!("3.2.1 -> Theoretical Review");
        //     println!("3.2.2 -> Peer Review");
        //     println!("3.3 -> Design Equivalence Claim");
        //     println!("3.4 -> Test Report");
        //     println!("3.5.3 -> Feedback Summary");
        //     println!("3.7.1 -> Biocompatibility Claim");
        //     println!("3.7.2 -> Cleaning Sterilisation Claim");
        // }
        _ => {
            println!("document number not recognised");
        }
    }

    let mut user_input = String::new();
    user_input.clear();

    let directory_to_search = dummy_to_run_menu();

    // println!("input path of family folder to be searched");

    // stdin.read_line(&mut user_input)?;

    // println!("input: {}", user_input);
    // remove quotes
    let re3 = Regex::new(r"\d{1,}").unwrap();

    // let mut largest_index: i32 = 0;
    let largest_index_path = String::new();

    println!(
        "Searching directory {}",
        directory_to_search.as_ref().unwrap().to_str().unwrap()
    );

    let mut docs: Vec<Document> = Vec::new();

    for file in WalkDir::new(
        directory_to_search
            .unwrap()
            .to_str()
            .unwrap()
            .trim()
            .replace("\"", ""),
    )
    .into_iter()
    .filter_map(|file| file.ok())
    .filter(|file| file.path().to_str().unwrap().contains(".docx"))
    {
        if file.metadata().unwrap().is_file() && re2.is_match(file.path().to_str().unwrap()) {
            // println!("{}", file.path().display());
            let file_path = file.path().to_owned();
            // println!("found: {}", file.path().display());
            let file_name = file.file_name().to_str().unwrap();

            let val = re2.find(file_name).unwrap();
            let doc_name_and_number = &file_name[val.start()..val.end()];
            let val2 = re3.find(doc_name_and_number).unwrap();
            let doc_number = &doc_name_and_number[val2.start()..val2.end()];
            // println!("{}", doc_number);

            let doc_number_int: i32 = doc_number.parse().unwrap();

            let doc = Document {
                index: doc_number_int,
                path: file_path,
            };

            docs.push(doc)
            // consumed_doc_numbers.push(doc_number_int);

            // println!("doc number as int: {}", doc_number_int);

            // if consumed_doc_numbers.contains(&doc_number_int) {
            //     println!("conflicting docs found")
            // } else if doc_number_int > largest_index {
            //     largest_index = doc_number_int;
            //     println!("new largest doc index: {}", doc_number);
            //     largest_index_path = file.path().display().to_string();
            // }
        }
    }

    let mut indices: HashMap<i32, Vec<Document>> = HashMap::new();
    // let mut indices: Vec<(i32, Vec<Document>)> = Vec::new();

    for doc in docs {
        println!(
            "found doc: {}",
            doc.path.file_name().unwrap().to_str().unwrap()
        );
        println!("with doc number {}", doc.index);
        println!("with path {}", doc.path.display());
        println!("");

        // let mut exists_in_vector = false;

        // for item in &indices {
        //     if item.0 == doc.index {
        //         item.1.push(doc.clone());
        //         exists_in_vector = true;
        //     }
        // }
        //
        // if !exists_in_vector {
        //     indices.push((doc.index, vec![doc.clone()]));
        // }

        if indices.contains_key(&doc.index) {
            let mut temp_vec = indices[&doc.index].clone();
            temp_vec.push(doc.clone());
            indices.insert(doc.index, temp_vec);
        } else {
            indices.insert(doc.index, vec![doc]);
        }
    }

    let mut indices_sorted: Vec<Vec<Document>> = Vec::new();
    let binding = indices.clone();
    let largest_index = binding.iter().max_by_key(|x| x.0);
    let mut largest_index_for_sorting: i32;

    // let indices_iter = indices.iter();

    while indices.iter().len() > 0 {
        largest_index_for_sorting = *indices.iter().max_by_key(|x| x.0).unwrap().0;
        indices_sorted.push(indices.remove(&largest_index_for_sorting).unwrap());
        // println!("sorting");
    }

    // for (index, docs) in indices.iter() {
    //     println!("documents at index: {}", index);
    //     for doc in docs.clone() {
    //         println!("{}", doc.path.display());
    //     }
    // }

    for document_list in indices_sorted.into_iter().enumerate() {
        let to_print = "document index:".yellow();
        println!(
            "{} {}",
            to_print,
            format!("{}", document_list.0.to_string().yellow())
        );

        if document_list.1.len() > 1 {
            println!("{}", "index contains multiple documents!".red().bold());
            for doc in document_list.1 {
                println!("{}", doc.path.display());
            }
        } else {
            println!("{}", document_list.1[0].path.display());
        }
    }

    println!("");
    println!("================================================================================================");
    println!("largest document index: {}", largest_index.unwrap().0);
    println!("at: {}", largest_index_path);

    pause();

    Ok(())
}
