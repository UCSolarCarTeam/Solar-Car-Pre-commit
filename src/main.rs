use std::env;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::Read;

// Function that takes in the commit message path and returns the commit message
fn get_commit_message(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

// Function to validate the commit message
fn validate_commit_message(commit_message: &str) -> io::Result<()> {
    let mut is_valid = 0;
    let mut line_index = 0;

    // Check if commit message starts with "SFT-XXX"
    if !commit_message.starts_with("SFT-") {
        return Err(Error::new(ErrorKind::Other,
            "Commit message must start with SFT-XXX"));
    }

    // Check if commit message has lines over 72 characters long
    let mut lines = commit_message.lines();
    while let Some(line) = lines.next() {
        if line_index != 0 && line.len() == 0 {
            break;
        }
        if line.len() > 72  {
            return Err(Error::new(ErrorKind::Other,
                "Commit message must not have lines over 72 characters long"));
        }
        if line.len() < 20 {
            println!("Length: {}", line.len());
            return Err(Error::new(ErrorKind::Other,
                "Commit message must not have lines under 20 characters long"));
        }
        line_index += 1;
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the commit message
    let commit_message = get_commit_message(".git/COMMIT_EDITMSG");

    std::process::exit(
        match validate_commit_message(&commit_message) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{}", err);
                1
            }
        }
    );
}
