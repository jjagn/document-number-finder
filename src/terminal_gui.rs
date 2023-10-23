use crate::{is_directory, FromStr, PathBuf};

use walkdir::WalkDir;

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError, Text,
};

#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
    lcp: String,
}

pub fn dummy_to_run_menu() -> Result<PathBuf, String> {
    let development_path = "H:\\Development";
    let _projects = WalkDir::new(development_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| is_directory(e));

    // let mut file_pick_menu_items = Vec::new();
    // file_pick_menu_items.push(button("Manually input folder path to search"));
    // file_pick_menu_items.push(label(""));
    // file_pick_menu_items.push(label("Select folder"));
    // file_pick_menu_items.push(label(""));
    //
    // for item in projects {
    //     let dir_name = item.unwrap();
    //     let dir_name2 = dir_name.path().file_name().unwrap().to_str().unwrap();
    //     file_pick_menu_items.push(button(dir_name2));
    // }
    //
    // let new_menu = menu(file_pick_menu_items);
    //
    // run(&new_menu);
    // let mm = mut_menu(&new_menu);
    // let mut dir_name = mm.selected_item_name();
    //
    // let mut user_input = String::new();
    // user_input.clear();
    // let stdin = io::stdin();

    // let current_dir = std::env::current_dir().unwrap();
    let current_dir = PathBuf::from_str(development_path).unwrap();
    let _help_message = format!("Current directory: {}", current_dir.to_string_lossy());

    let ans = Text::new("Development Folder to search:")
        .with_autocomplete(FilePathCompleter::default())
        // .with_help_message(&help_message)
        .prompt();

    match ans {
        Ok(path) => Ok(PathBuf::from_str(path.as_str()).unwrap()),
        Err(error) => Err(error.to_string()),
    }

    // if dir_name == "Manually input folder path to search" {
    //     println!("input path of family folder to be searched");
    //     stdin.read_line(&mut user_input).unwrap();
    //     println!("input: {}", user_input);
    //     user_input = user_input.trim().replace("\"", "");
    //     dir_name = user_input.as_str();
    //     return Ok(PathBuf::from_str(dir_name).unwrap());
    // } else {
    //     let projects2 = WalkDir::new(development_path)
    //         .min_depth(1)
    //         .max_depth(1)
    //         .into_iter()
    //         .filter_entry(|e| is_directory(e));
    //
    //     for item in projects2 {
    //         let dir_name_check_option = item.unwrap();
    //         let dir_name_check = dir_name_check_option
    //             .path()
    //             .file_name()
    //             .unwrap()
    //             .to_str()
    //             .unwrap();
    //
    //         if dir_name_check == dir_name {
    //             return Ok(dir_name_check_option.into_path());
    //         }
    //     }
    //     Err("path not found".to_string())
    // }
}

impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        self.input = input.to_owned();
        self.paths.clear();

        // let input_path = std::path::PathBuf::from(input);

        // let fallback_parent = input_path
        //     .parent()
        //     .map(|p| {
        //         if p.to_string_lossy() == "" {
        //             std::path::PathBuf::from(".")
        //         } else {
        //             p.to_owned()
        //         }
        //     })
        //     .unwrap_or_else(|| std::path::PathBuf::from("."));

        // let scan_dir = if input.ends_with('/') {
        //     input_path
        // } else {
        //     fallback_parent.clone()
        // };

        let scan_dir = "H:\\Development";

        let entries = match std::fs::read_dir(scan_dir) {
            Ok(read_dir) => Ok(read_dir),
            // Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback_parent),
            Err(err) => Err(err),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        // let development_path = "H:\\Development";
        // let projects = WalkDir::new(development_path)
        //     .min_depth(1)
        //     .max_depth(1)
        //     .into_iter()
        //     .filter_entry(|e| is_directory(e));
        //
        // let entries: DirEntry = projects.collect();

        let mut idx = 0;
        let limit = 15;

        while idx < entries.len() && self.paths.len() < limit {
            let entry = entries.get(idx).unwrap();

            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}/", path.to_string_lossy())
            } else {
                path.to_string_lossy().to_string()
            };

            if path_str.to_lowercase().contains(&self.input.to_lowercase())
                && path_str.len() != self.input.len()
            {
                self.paths.push(path_str);
            }

            idx = idx.saturating_add(1);
        }

        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.paths.clone();
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

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        // Ok(match highlighted_suggestion {
        //     Some(suggestion) => Replacement::Some(suggestion),
        //     None => match self.lcp.is_empty() {
        //         true => match self.paths.first() {
        //             Some(path) => Replacement::Some(path.to_owned()),
        //             None => Replacement::None,
        //         },
        //         false => Replacement::Some(self.lcp.clone()),
        //     },
        // })

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.paths.first() {
                Some(path) => Replacement::Some(path.to_owned()),
                None => Replacement::None,
            },
        })
    }
}
