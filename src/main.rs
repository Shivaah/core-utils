use std::collections::HashSet;
use std::fs::{self, DirEntry};
use std::io::{self};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = String::from(line?.trim());
        let tokens = scan(input);

        if let Some((command, args)) = parse(tokens) {
            let continue_execution = execute_command(command.as_str(), args)?;

            if !continue_execution {
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Scan an input string and split it into a vector of tokens.
///
/// This function takes an input string `input` and splits it into individual tokens based on spaces.
///
/// # Arguments
///
/// * `input` - A string representing the input to be scanned and split into tokens.
fn scan(input: String) -> Vec<String> {
    input.split(" ").map(String::from).collect()
}

/// Parse a vector of tokens into a command and its arguments.
///
/// This function takes a vector of strings `tokens` representing command tokens. It extracts the first token
/// as the command and the rest as its arguments.
///
/// # Arguments
///
/// * `tokens` - A vector of strings representing the command tokens.
fn parse(tokens: Vec<String>) -> Option<(String, Vec<String>)> {
    if let Some(x) = tokens.get(0) {
        let command = x.to_string();
        let args: Vec<String> = tokens.into_iter().skip(1).collect();

        Some((command, args))
    } else {
        None
    }
}

/// Execute a command with the provided arguments.
///
/// This function takes a command string `command` and a vector of strings `args` representing the arguments
/// for the command. It performs the logic for executing the specified command and returns an `io::Result<bool>`.
///
/// # Arguments
///
/// * `command` - A string representing the name of the command to execute.
/// * `args` - A vector of strings representing the arguments for the command.
fn execute_command(command: &str, args: Vec<String>) -> io::Result<bool> {
    if command.is_empty() {
        print!("");
    }

    match command {
        "echo" => execute_echo(args),
        "exit" => execute_exit(),
        "ls" => execute_ls(args),
        _ => {
            eprintln!("Command not found : {}", command);
            Ok(true)
        }
    }
}

/// Execute the `echo` command with the provided arguments.
///
/// This function takes a vector of strings `args` representing the arguments passed to the `echo` command.
/// 
/// It performs the logic for the `echo` linux command.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the arguments for the `echo` command.
fn execute_echo(args: Vec<String>) -> io::Result<bool> {
    let result = String::from(args.join(" ").trim());

    println!("{0}", result);

    Ok(true)
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
fn execute_ls(args: Vec<String>) -> Result<bool, io::Error> {
    let (path, options): (String, Option<HashSet<char>>) = if args.len() == 0 {
        (String::from("."), None)
    } else {
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
    };

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
                            entries
                                .into_iter()
                                .for_each(|e| println!("{}", e.path().display()));
                        }
                    }
                }
            }

            return Ok(true);
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => eprintln!("No such file or directory: {}", path),
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied to view contents of: {}", path)
                }
                _ => eprintln!("File is not a directory: {}", path),
            }

            return Ok(true);
        }
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

/// Terminate the application.
///
/// This function is responsible for gracefully terminating the application. It prints a "Goodbye!" message
/// and returns `Ok(false)`.
fn execute_exit() -> io::Result<bool> {
    println!("Goodbye!");

    Ok(false)
}
