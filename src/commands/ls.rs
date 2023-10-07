use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io,
    os::unix::prelude::{MetadataExt, PermissionsExt},
    path::PathBuf,
};

use crate::unix::permissions;

/// Execute the `ls` command with the provided arguments.
///
/// This function takes a vector of strings `args` representing the arguments passed to the `ls` command.
///
/// It performs the logic for the `ls` linux command.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the arguments for the `ls` command.
pub fn execute(args: Vec<String>) -> Result<bool, io::Error> {
    let (path, options) = parse(args);

    match fs::read_dir(path.clone()) {
        Ok(read_dir) => {
            let mut errors = vec![];

            let entries: Vec<DirEntry> = read_dir
                .filter_map(|entry: Result<DirEntry, io::Error>| {
                    entry.map_err(|e: io::Error| errors.push(e)).ok()
                })
                .collect();

            if errors.len() > 0 {
                errors
                    .into_iter()
                    .for_each(|e| println!("{}", e.to_string()));
                return Ok(true);
            }

            match options {
                None => {
                    let out: String = entries
                        .into_iter()
                        .map(|e| e.path().display().to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

                    println!("{}", out);
                }
                Some(letters) => {
                    if let Err(wrong_option) = validate_ls_options(&letters) {
                        println!("ls : invalid option - '{}'", wrong_option);
                    } else {
                        if letters.contains(&'l') {
                            entries.into_iter().for_each(|e| {
                                let metadata: fs::Metadata = e.metadata().unwrap();
                                let path: PathBuf = e.path();
                                let file_type = metadata.file_type();

                                println!(
                                    "{0} {1:o} {2}",
                                    if file_type.is_dir() { "d" } else { "-" },
                                    metadata.mode(),
                                    path.display()
                                )
                            });
                        }
                    }
                }
            }

            return Ok(true);
        }
        Err(e) => handle_error(e, path),
    }
}

fn handle_error(error: io::Error, path: String) -> io::Result<bool> {
    match error.kind() {
        io::ErrorKind::NotFound => eprintln!("no such file or directory: {}", path),
        io::ErrorKind::PermissionDenied => {
            eprintln!("permission denied to view contents of: {}", path)
        }
        _ => eprintln!("file is not a directory: {}", path),
    }

    return Ok(true);
}

fn parse(args: Vec<String>) -> (String, Option<HashSet<char>>) {
    if args.len() == 0 {
        return (String::from("."), None);
    }

    let first_arg = args.get(0).unwrap();

    if first_arg.starts_with("-") {
        if let Some(letters) = first_arg.get(1..) {
            let mut uniques: HashSet<char> = HashSet::new();

            letters
                .chars()
                .collect::<Vec<char>>()
                .retain(|e| uniques.insert(*e));

            if let Some(second_arg) = args.get(1) {
                (second_arg.to_string(), Some(uniques))
            } else {
                (String::from("."), Some(uniques))
            }
        } else {
            (first_arg.to_string(), None)
        }
    } else {
        (first_arg.to_string(), None)
    }
}

/// Validates the provided options for the `ls` command.
///
/// This function takes a reference to a `HashSet<char>` containing the options for the `ls` command.
///
/// It checks if each option is valid and only allows the option 'l' for the moment.
///
/// # Arguments
///
/// * `options` - A reference to a `HashSet<char>` containing the options for the `ls` command.
fn validate_ls_options(options: &HashSet<char>) -> Result<(), &char> {
    let valid_options = ['l'];

    for (_, option) in options.iter().enumerate() {
        if !valid_options.contains(option) {
            return Err(option);
        }
    }

    return Ok(());
}
