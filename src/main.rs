use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap, io, path::PathBuf};
use terminal_link::Link;

use colored::*;

use regex::Regex;
use terminal_menu::{button, label, menu, mut_menu, run};
use walkdir::{DirEntry, WalkDir};

// import and create modules
use crate::document::*;
use helpers::*;
use terminal_gui::*;

pub mod document;
pub mod helpers;
pub mod terminal_gui;

extern crate walkdir;

fn main() -> io::Result<()> {
    let menu = menu(vec![
        label("select document code:"),
        button("1.5.1 -> Packaging Selection"),
        button("2.1 -> Usability Assessment"),
        button("3.2.1 -> Theoretical Review"),
        button("3.2.2 -> Peer Review"),
        button("3.4 -> Test Report"),
        button("3.5.3 -> Feedback Summary"),
        button("3.7.1 -> Biocompatibility Assessment"),
        button("All equivalence claims"),
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
        "1.5.1" => re2 = Regex::new(r"PackagingSelection\d+").unwrap(),
        "2.1" => re2 = Regex::new(r"UsabilityAssess\d+").unwrap(),
        "3.2.1" => re2 = Regex::new(r"TheoreticalReview\d+").unwrap(),
        "3.2.2" => re2 = Regex::new(r"PeerReview\d+").unwrap(),
        "3.4" => re2 = Regex::new(r"TestReport\d+").unwrap(),
        "3.5.3" => re2 = Regex::new(r"FeedbackSummary\d+").unwrap(),
        "3.7.1" => re2 = Regex::new(r"BiocompAssess\d+").unwrap(),
        "All" => re2 = Regex::new(r"\D+Claim\d+").unwrap(),
        _ => {
            println!("document number not recognised");
        }
    }

    let directory_to_search = dummy_to_run_menu();

    let re3 = Regex::new(r"\d{1,}").unwrap();

    println!(
        "Searching directory {}",
        directory_to_search.as_ref().unwrap().display()
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
    .filter(|file| !file.path().to_str().unwrap().contains("_Archive"))
    .filter(|f| !f.path().to_str().unwrap().contains("~$"))
    {
        if file.metadata().unwrap().is_file() && re2.is_match(file.path().to_str().unwrap()) {
            let file_path = file.path().to_owned();
            let file_name = file.file_name().to_str().unwrap();

            let val = re2.find(file_name).unwrap();
            let doc_name_and_number = &file_name[val.start()..val.end()];
            let val2 = re3.find(doc_name_and_number).unwrap();
            let doc_number = &doc_name_and_number[val2.start()..val2.end()];

            let doc_number_int: i32 = doc_number.parse().unwrap();

            let doc = Document {
                index: doc_number_int,
                path: file_path,
            };

            docs.push(doc)
        }
    }

    let mut indices: HashMap<i32, Vec<Document>> = HashMap::new();

    for doc in docs {
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

    let mut indices_with_error: Vec<i32> = Vec::new();
    let mut all_indices: Vec<i32> = Vec::new();

    while indices.iter().len() > 0 {
        largest_index_for_sorting = *indices.iter().max_by_key(|x| x.0).unwrap().0;
        indices_sorted.push(indices.remove(&largest_index_for_sorting).unwrap());
    }

    for document_list in indices_sorted.iter().rev() {
        println!("");
        let to_print = "document index:".yellow();
        all_indices.push(document_list[0].index);
        println!(
            "{} {}",
            to_print,
            format!("{}", document_list[0].index.to_string().yellow())
        );

        if document_list.len() > 1 {
            println!("{}", "index contains multiple documents!".red().bold());
            indices_with_error.push(document_list[0].index);
            for doc in document_list {
                // let path_without_initial = doc_path_without_top_level_dirs(doc.clone());
                let link = Link::new(
                    // path_without_initial.as_str(),
                    doc.path.to_str().unwrap(),
                    doc.path.parent().unwrap().to_str().unwrap(),
                );
                println!("{}", link);
            }
        } else {
            let doc = document_list[0].clone();
            // let path_without_initial = doc_path_without_top_level_dirs(doc.clone());

            let link = Link::new(
                // path_without_initial.as_str(),
                doc.path.to_str().unwrap(),
                doc.path.parent().unwrap().to_str().unwrap(),
            );
            println!("{}", link);
        }
    }

    let num_errors = indices_with_error.len();
    println!("=========================================================================");

    if num_errors > 0 {
        println!("{}", "document numbering errors detected!".red().bold());
        println!("=====================================================================");
        println!("number of doubled up indices detected: {}", num_errors);

        for document_list in indices_sorted.iter().rev() {
            if indices_with_error.contains(&document_list[0].index) {
                println!("index {}", document_list[0].index);
                for document in document_list {
                    println!("{}", document.path.display());
                }
            }
        }
        println!("");
    }

    println!("");
    println!("=========================================================================");
    println!("largest document index: {}", largest_index.unwrap().0);

    println!(
        "suggested index for next document: {}",
        largest_index.unwrap().0 + 1
    );

    pause();

    Ok(())
}
