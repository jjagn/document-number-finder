use std::path::Path;
use std::str::FromStr;
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

fn doc_path_without_top_level_dirs(doc: Document) -> String {
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

fn dummy_to_run_menu() -> Result<PathBuf, String> {
    let development_path = "H:\\Development";
    let projects = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    let mut file_pick_menu_items = Vec::new();
    file_pick_menu_items.push(button("Manually input folder path to search"));
    file_pick_menu_items.push(label(""));
    file_pick_menu_items.push(label("Select folder"));
    file_pick_menu_items.push(label(""));

    for item in projects {
        let dir_name = item.unwrap();
        let dir_name2 = dir_name.path().file_name().unwrap().to_str().unwrap();
        // println!("{}", item?.path().file_name().unwrap().to_str().unwrap());
        file_pick_menu_items.push(button(dir_name2));
    }

    let new_menu = menu(file_pick_menu_items);

    run(&new_menu);
    let mm = mut_menu(&new_menu);
    let mut dir_name = mm.selected_item_name();

    let mut user_input = String::new();
    user_input.clear();
    let stdin = io::stdin();

    if dir_name == "Manually input folder path to search" {
        println!("input path of family folder to be searched");
        stdin.read_line(&mut user_input).unwrap();
        println!("input: {}", user_input);
        user_input = user_input.trim().replace("\"", "");
        dir_name = user_input.as_str();
        return Ok(PathBuf::from_str(dir_name).unwrap());
    } else {
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
}

fn main() -> io::Result<()> {
    let menu = menu(vec![
        label("select document code:"),
        button("1.5.1 -> Packaging Selection"),
        button("2.1 -> Usability Assessment"),
        button("3.2.1 -> Theoretical Review"),
        button("3.2.2 -> Peer Review"),
        // button("3.3 -> Design Equivalence Claim"),
        button("3.4 -> Test Report"),
        button("3.5.3 -> Feedback Summary"),
        button("3.7.1 -> Biocompatibility Assessment"),
        // button("3.7.2 -> Cleaning Sterilisation Claim"),
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
        // note that usability assment naming is changing in future
        "2.1" => re2 = Regex::new(r"UsabilityAssess\d+").unwrap(),
        "3.2.1" => re2 = Regex::new(r"TheoreticalReview\d+").unwrap(),
        "3.2.2" => re2 = Regex::new(r"PeerReview\d+").unwrap(),
        // "3.3" => re2 = Regex::new(r"DesignEquivClaim\d+").unwrap(),
        "3.4" => re2 = Regex::new(r"TestReport\d+").unwrap(),
        "3.5.3" => re2 = Regex::new(r"FeedbackSummary\d+").unwrap(),
        "3.7.1" => re2 = Regex::new(r"BiocompAssess\d+").unwrap(),
        // "3.7.2" => re2 = Regex::new(r"CleaningSteriClaim\d+").unwrap(),
        "All" => re2 = Regex::new(r"\D+Claim\d+").unwrap(),
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

    // let mut last_index = 1;
    // let mut skipped_indices: Vec<i32> = Vec::new();
    //
    // for index in all_indices.iter() {
    //     // println!("index: {}", index);
    //     if *index > last_index + 1 {
    //         // println!("last index: {}", last_index);
    //         // println!("current index: {}", *index);
    //         let mut skipped_index = last_index + 1;
    //         while skipped_index < *index {
    //             skipped_indices.push(skipped_index);
    //             skipped_index += 1;
    //         }
    //     }
    //     last_index = *index;
    // }
    //
    // if skipped_indices.len() > 0 {
    //     println!("indices skipped!");
    //     for skipped_index in skipped_indices.iter() {
    //         println!("index {} skipped", skipped_index);
    //     }
    // }

    println!("");
    println!("=========================================================================");
    println!("largest document index: {}", largest_index.unwrap().0);

    println!(
        "suggested index for next document: {}",
        largest_index.unwrap().0 + 1
    );

    // if skipped_indices.len() > 0 {
    //     println!("OR lowest unused/skipped index: {}", skipped_indices[0]);
    // }
    pause();

    Ok(())
}
