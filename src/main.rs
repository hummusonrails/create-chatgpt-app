use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: create-chatgpt-app my-app");
        return;
    }

    let app_name = &args[1];

    // Clone the template repository
    let git_status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/hummusonrails/chatgpt-rocket-template.git")
        .arg(app_name)
        .status()
        .expect("Failed to clone the template repository");

    if !git_status.success() {
        eprintln!("Error cloning the template repository");
        return;
    }

    // Change to the newly created directory
    let cd_status = Command::new("cd")
        .arg(app_name)
        .status()
        .expect("Failed to change directory");

    if !cd_status.success() {
        eprintln!("Error changing to the new directory");
        return;
    }
}
