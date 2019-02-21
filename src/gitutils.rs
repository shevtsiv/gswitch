use std::process::Command;

pub fn get_git_name() -> String {
    let git_name = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .expect("Failed to execute 'git config --get user.name' command!");
    let stdout_len = git_name.stdout.len() - 1; // We have to eliminate the last one byte as it is `\n`
    String::from_utf8(git_name.stdout[..stdout_len].to_vec())
        .expect("Command 'git config --get user.name' has returned invalid UTF-8 output!")
}

pub fn set_git_name(new_name: &str) {
    Command::new("git")
        .args(&["config", "--global", "user.name", new_name])
        .spawn()
        .expect("Failed to execute 'git config --global user.name' command!");
}

pub fn get_git_email() -> String {
    let git_email = Command::new("git")
        .args(&["config", "--get", "user.email"])
        .output()
        .expect("Failed to execute 'git config --get user.email' command!");
    let stdout_len = git_email.stdout.len() - 1; // We have to eliminate the last one byte as it is `\n`
    String::from_utf8(git_email.stdout[..stdout_len].to_vec())
        .expect("Command 'git config --get user.email' has returned invalid UTF-8 output!")
}

pub fn set_git_email(new_email: &str) {
    Command::new("git")
        .args(&["config", "--global", "user.email", new_email])
        .spawn()
        .expect("Failed to execute 'git config --global user.email' command!");
}
