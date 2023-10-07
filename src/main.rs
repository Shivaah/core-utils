use std::io::{self};

mod commands;
mod unix;

use commands::{
    echo::execute as execute_echo, exit::execute as execute_exit, ls::execute as execute_ls,
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = String::from(line?.trim());
        let tokens = scan(input);

        if let Some((command_name, args)) = parse(tokens) {
            let continue_execution = execute_command(command_name, args)?;

            if !continue_execution {
                return Ok(());
            }
        }
    }

    Ok(())
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
fn execute_command(command_name: String, args: Vec<String>) -> io::Result<bool> {
    if command_name.is_empty() {
        print!("");
    }

    match command_name.as_str() {
        "echo" => execute_echo(args),
        "exit" => execute_exit(),
        "ls" => execute_ls(args),
        _ => {
            eprintln!("command not found : {}", command_name);
            Ok(true)
        }
    }
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
/// as the command name and the rest as its arguments.
///
/// # Arguments
///
/// * `tokens` - A vector of strings representing the command tokens.
fn parse(tokens: Vec<String>) -> Option<(String, Vec<String>)> {
    if let Some(x) = tokens.get(0) {
        let command_name = x.to_string();
        let args: Vec<String> = tokens.into_iter().skip(1).collect();

        Some((command_name, args))
    } else {
        None
    }
}
