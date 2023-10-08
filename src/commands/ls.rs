use std::{
    collections::HashSet,
    fs::{self, DirEntry, ReadDir},
    io,
    os::{linux::fs::MetadataExt, unix::prelude::FileTypeExt},
    path::PathBuf,
};

use crate::unix::permissions::UnixPermissions;

struct FileType(std::fs::FileType);

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_type = self.0;

        write!(
            f,
            "{}",
            match file_type {
                _ if file_type.is_dir() => 'd',
                _ if file_type.is_symlink() => 's',
                _ if file_type.is_block_device() => 'b',
                _ if file_type.is_char_device() => 'c',
                _ if file_type.is_socket() => 's',
                _ if file_type.is_fifo() => 'p',
                _ => '-',
            }
        )
    }
}

/// Execute the `ls` command with the provided arguments.
///
/// This function takes a vector of strings `args` representing the arguments passed to the `ls` command.
///
/// It performs the logic for the `ls` linux command.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the arguments for the `ls` command.
pub fn execute(args: Vec<String>) -> io::Result<bool> {
    let (path, options) = parse(args);

    if let Err(wrong_option) = validate_ls_options(&options) {
        println!("ls : invalid option - '{}'", wrong_option);

        return Ok(true);
    }

    match fs::read_dir(path.clone()) {
        Ok(read_dir) => {
            let entries = read_entries(read_dir);

            if let Err(errors) = entries {
                errors
                    .into_iter()
                    .for_each(|e| println!("{}", e.to_string()));

                return Ok(true);
            }

            let entries = entries.unwrap();

            if options.len() == 0 {
                let out: String = entries
                    .into_iter()
                    .map(|e| e.path().display().to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                println!("{}", out);

                return Ok(true);
            }

            if options.contains(&'l') {
                entries.into_iter().for_each(|e| {
                    let metadata: fs::Metadata = e.metadata().unwrap();
                    let path: PathBuf = e.path();
                    let permissions = metadata.permissions();

                    let permissions_str = format!(
                        "{}{}{}",
                        permissions.owner(),
                        permissions.group(),
                        permissions.other()
                    );

                    println!(
                        "{}{} {} {} {} {}",
                        FileType(metadata.file_type()),
                        permissions_str,
                        metadata.st_uid(),
                        metadata.st_gid(),
                        metadata.st_size(),
                        path.display()
                    )
                });
            }

            return Ok(true);
        }
        Err(e) => handle_error(e, path),
    }
}

fn read_entries(read_dir: ReadDir) -> Result<Vec<DirEntry>, Vec<io::Error>> {
    let mut errors = vec![];

    let entries: Vec<DirEntry> = read_dir
        .filter_map(|entry: Result<DirEntry, io::Error>| {
            entry.map_err(|e: io::Error| errors.push(e)).ok()
        })
        .collect();

    if errors.len() > 0 {
        return Err(errors);
    }

    return Ok(entries);
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

fn parse(args: Vec<String>) -> (String, HashSet<char>) {
    let mut uniques: HashSet<char> = HashSet::new();

    if args.len() == 0 {
        return (String::from("."), uniques);
    }

    let first_arg = args.get(0).unwrap();

    if first_arg.starts_with("-") {
        if let Some(letters) = first_arg.get(1..) {
            letters
                .chars()
                .collect::<Vec<char>>()
                .retain(|e| uniques.insert(*e));

            if let Some(second_arg) = args.get(1) {
                (second_arg.to_string(), uniques)
            } else {
                (String::from("."), uniques)
            }
        } else {
            (first_arg.to_string(), uniques)
        }
    } else {
        (first_arg.to_string(), uniques)
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

    if options.is_empty() {
        return Ok(());
    }

    for (_, option) in options.iter().enumerate() {
        if !valid_options.contains(option) {
            return Err(option);
        }
    }

    return Ok(());
}
