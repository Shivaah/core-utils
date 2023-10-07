use std::io;

/// Terminate the application.
///
/// This function is responsible for gracefully terminating the application. It prints a "Goodbye!" message
/// and returns `Ok(false)`.
pub fn execute() -> io::Result<bool> {
    println!("Goodbye!");

    Ok(false)
}
