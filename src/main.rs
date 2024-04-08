use egui::TextBuffer;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::{collections::HashMap, io, path::PathBuf};
use terminal_link::Link;

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError, Text,
};

use colored::*;

use regex::Regex;
// use terminal_menu::{button, label, menu, mut_menu, run};
use walkdir::{DirEntry, WalkDir};

// import and create modules
use crate::document::*;
use helpers::*;
use terminal_gui::*;

pub mod document;
pub mod helpers;
pub mod terminal_gui;

extern crate walkdir;

#[derive(Clone, Default)]
pub struct FileTypeCompleter {
    input: String,
    docs: Vec<String>,
    lcp: String,
}

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    println!("Drag the dhf you'd like to search to this window, then press enter");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{} will be searched", input);
        }
        Err(error) => println!("error: {error}"),
    }

    let development_path = input;

    // let help_message = "Start typing the document type you'd like to search. Tab to autocomplete, arrow keys to select, then enter to submit.";

    let mut successful_match = false;

    let mut re2 = Regex::new("").unwrap();

    while !successful_match {
        successful_match = true;
        let user_input = Text::new("Document to search for:")
            .with_autocomplete(FileTypeCompleter::default())
            // .with_help_message(&help_message)
            .prompt();

        let mut doc_input = match user_input {
            Ok(doc_input) => doc_input,
            Err(_) => "error!".to_string(),
        };

        match doc_input.trim() {
            "1.5.1 Packaging Selection" => re2 = Regex::new(r"PackagingSelection\d+").unwrap(),
            "2.1 Usability Assessment" => re2 = Regex::new(r"UsabilityAssess\d+").unwrap(),
            "3.2.1 Theoretical Review" => re2 = Regex::new(r"TheoreticalReview\d+").unwrap(),
            "3.2.2 Peer Review" => re2 = Regex::new(r"PeerReview\d+").unwrap(),
            "3.4 Test Report" => re2 = Regex::new(r"TestReport\d+").unwrap(),
            "3.5.3 Feedback Summary" => re2 = Regex::new(r"FeedbackSummary\d+").unwrap(),
            "3.7.1 Biocompatibility Assessment" => re2 = Regex::new(r"BiocompAssess\d+").unwrap(),
            "All equivalence claims (Cleaning, Design, Biocomp)" => {
                re2 = Regex::new(r"\D+Claim\d+").unwrap()
            }
            _ => {
                println!("document not recognised");
                let possible_matches = vec![
                    "1.5.1 Packaging Selection",
                    "2.1 Usability Assessment",
                    "3.2.1 Theoretical Review",
                    "3.2.2 Peer Review",
                    "3.4 Test Report",
                    "3.5.3 Feedback Summary",
                    "3.7.1 Biocompatibility Assessment",
                    "All equivalence claims (Cleaning, Design, Biocomp)",
                ];

                println!("{}", doc_input);

                for item in possible_matches.into_iter() {
                    println!("{}", item);
                    if item.to_lowercase().contains(&doc_input) {
                        doc_input = item.to_string();
                        println!("falling back to first matched: {}", doc_input);
                    }
                }

                match doc_input.as_str() {
                    "1.5.1 Packaging Selection" => {
                        re2 = Regex::new(r"PackagingSelection\d+").unwrap()
                    }
                    "2.1 Usability Assessment" => re2 = Regex::new(r"UsabilityAssess\d+").unwrap(),
                    "3.2.1 Theoretical Review" => {
                        re2 = Regex::new(r"TheoreticalReview\d+").unwrap()
                    }
                    "3.2.2 Peer Review" => re2 = Regex::new(r"PeerReview\d+").unwrap(),
                    "3.4 Test Report" => re2 = Regex::new(r"TestReport\d+").unwrap(),
                    "3.5.3 Feedback Summary" => re2 = Regex::new(r"FeedbackSummary\d+").unwrap(),
                    "3.7.1 Biocompatibility Assessment" => {
                        re2 = Regex::new(r"BiocompAssess\d+").unwrap()
                    }
                    "All equivalence claims (Cleaning, Design, Biocomp)" => {
                        re2 = Regex::new(r"\D+Claim\d+").unwrap()
                    }
                    _ => successful_match = false,
                }
            }
        }
    }

    // let directory_to_search = dummy_to_run_menu(&development_path);
    let directory_to_search = PathBuf::from_str(&development_path);

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

    let largest_index_unwrapped = match largest_index {
        Some(x) => *x.0,
        None => 0,
    };

    if largest_index_unwrapped >= 1 {
        println!("");
        println!("=========================================================================");
        println!("largest document index: {}", largest_index_unwrapped);

        println!(
            "suggested index for next document: {}",
            largest_index_unwrapped + 1
        );
    } else {
        println!("");
        println!("=========================================================================");
        println!("no docs of the type you're searching for were found");
        println!("suggested index for next document: 1");
    }

    pause();

    Ok(())
}

impl FileTypeCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        let entries = vec![
            "1.5.1 Packaging Selection",
            "2.1 Usability Assessment",
            "3.2.1 Theoretical Review",
            "3.2.2 Peer Review",
            "3.4 Test Report",
            "3.5.3 Feedback Summary",
            "3.7.1 Biocompatibility Assessment",
            "All equivalence claims (Cleaning, Design, Biocomp)",
        ];

        self.input = input.to_owned();
        self.docs.clear();

        let mut idx = 0;
        let limit = 15;

        while idx < entries.len() && self.docs.len() < limit {
            let entry_address = entries.get(idx).unwrap();

            let entry = entry_address.to_string();

            // autocomplete matching logic
            // if entry.starts_with(&self.input) && entry.len() != self.input.len() {
            //     self.docs.push(entry);
            // }

            if entry.to_lowercase().contains(&self.input.to_lowercase())
                && entry.len() != self.input.len()
            {
                self.docs.push(entry);
            }

            idx = idx.saturating_add(1);
        }

        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.docs.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret;
        }

        let mut first_word = sorted.first().unwrap().chars();
        let mut last_word = sorted.last().unwrap().chars();

        loop {
            match (first_word.next(), last_word.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    ret.push(c1);
                }
                _ => return ret,
            }
        }
    }
}

impl Autocomplete for FileTypeCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.docs.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.lcp.is_empty() {
                true => match self.docs.first() {
                    Some(doc) => Replacement::Some(doc.to_string()),
                    None => Replacement::None,
                },
                false => Replacement::Some(self.lcp.clone()),
            },
        })
    }
}
