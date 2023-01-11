use std::io;
use std::io::prelude::*;

use regex::Regex;
use walkdir::WalkDir;

extern crate walkdir;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut re2 = Regex::new("").unwrap();

    let mut repeat_doc_input: bool = true;
    while repeat_doc_input {
        repeat_doc_input = false;
        println!("input id number of document you'd like to index (i.e. 3.4 for test report), then press enter");
        println!("input h to list doc codes");

        stdin.read_line(&mut user_input)?;
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
            "h" | "H" => {
                println!("1.5.1 -> Packaging Selection");
                println!("2.1 -> Usability Assessment");
                println!("3.2.1 -> Theoretical Review");
                println!("3.2.2 -> Peer Review");
                println!("3.3 -> Design Equivalence Claim");
                println!("3.4 -> Test Report");
                println!("3.5.3 -> Feedback Summary");
                println!("3.7.1 -> Biocompatibility Claim");
                println!("3.7.2 -> Cleaning Sterilisation Claim");
                repeat_doc_input = true;
            }
            _ => {
                println!("document number not recognised");
                repeat_doc_input = true;
            }
        }
        user_input.clear();
    }

    println!("input path of family folder to be searched");

    stdin.read_line(&mut user_input)?;

    println!("input: {}", user_input);
    // remove quotes
    let re3 = Regex::new(r"\d{1,}").unwrap();

    let mut largest_index: i32 = 0;
    let mut largest_index_path = String::new();

    for file in WalkDir::new(user_input.trim().replace("\"", ""))
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() && re2.is_match(file.path().to_str().unwrap()) {
            // println!("{}", file.path().display());
            println!("found: {}", file.path().display());
            let file_name = file.file_name().to_str().unwrap();

            let val = re2.find(file_name).unwrap();
            let doc_name_and_number = &file_name[val.start()..val.end()];
            let val2 = re3.find(doc_name_and_number).unwrap();
            let doc_number = &doc_name_and_number[val2.start()..val2.end()];
            // println!("{}", doc_number);

            let doc_number_int: i32 = doc_number.parse().unwrap();

            // println!("doc number as int: {}", doc_number_int);

            if doc_number_int > largest_index {
                largest_index = doc_number_int;
                println!("new largest doc index: {}", doc_number);
                largest_index_path = file.path().display().to_string();
            }
        }
    }

    println!("");
    println!("================================================================================================");
    println!("largest document index: {}", largest_index);
    println!("at: {}", largest_index_path);

    pause();

    Ok(())
}
