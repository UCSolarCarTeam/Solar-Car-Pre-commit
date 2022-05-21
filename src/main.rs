use std::env;
use std::io::Read;

// Function that takes in the commit message path and returns the commit message
fn get_commit_message(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

// Function to validate the commit message
fn validate_commit_message(commit_message: &str) -> i8 {
    let mut is_valid = 0;

    // Check if commit message starts with "SFT-XXX"
    if !commit_message.starts_with("SFT-") {
        println!("Commit message must start with SFT-XXX");
        is_valid = 1;
    }

    // Check if commit message has lines over 72 characters long
    let mut lines = commit_message.lines();
    while let Some(line) = lines.next() {
        if line.len() > 72 {
            println!("Commit message must not have lines over 72 characters long");
            is_valid = 1;
        }
    }

    return is_valid;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the commit message
    let commit_message = get_commit_message(&args[1]);

    validate_commit_message(&commit_message);
}
