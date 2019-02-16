use std::process::Command;

fn main() {
    let new_name = "SomeName";
    let new_email = "someEmail@example.com";

    Command::new("git")
        .args(&["config", "--global", "user.name", format!("\"{}\"", new_name).as_str()])
        .output()
        .expect("Failed to execute 'git config --global user.name' command!");
    println!("Command 'git config --global user.name' has been executed successfully!");
    Command::new("git")
        .args(&["config", "--global", "user.email", format!("\"{}\"", new_email).as_str()])
        .output()
        .expect("Failed to execute 'git config --global user.email' command!");
    println!("Command 'git config --global user.email' has been executed successfully!");
    let confirm_name = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .expect("Failed to execute 'git config --get user.name' command!");
    println!("Command 'git config --get user.name' has been executed successfully!");
    let confirm_email = Command::new("git")
        .args(&["config", "--get", "user.email"])
        .output()
        .expect("Failed to execute 'git config --get user.email' command!");
    println!("Command 'git config --get user.email' has been executed successfully!");
    println!("Your new credentials: ");
    println!("Name: {}", String::from_utf8_lossy(&confirm_name.stdout[..confirm_name.stdout.len() - 1]));
    println!("Email: {}", String::from_utf8_lossy(&confirm_email.stdout));
}
