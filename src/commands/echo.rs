use std::io;

/// Execute the `echo` command with the provided arguments.
///
/// This function takes a vector of strings `args` representing the arguments passed to the `echo` command.
///
/// It performs the logic for the `echo` linux command.
///
/// # Arguments
///
/// * `args` - A vector of strings representing the arguments for the `echo` command.
pub fn execute(args: Vec<String>) -> io::Result<bool> {
    let result = String::from(args.join(" ").trim());

    println!("{0}", result);

    Ok(true)
}
