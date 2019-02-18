#[macro_use]
extern crate serde_derive;

mod config;

use std::process::Command;
use std::fs::File;
use std::env;

fn main() {
    let config_file = File::open("settings.toml").expect("Failed to open config file!");
    let config = config::init_config(config_file);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: gswitch <name>");
        return;
    }
    let nickname = args.get(1).unwrap();
    let account = config.get_account_by_name(nickname);
    if account.is_none() {
        println!("There is no Git account with such name!");
        return;
    }
    let account = account.unwrap();
    let new_name = account.get_name();
    let new_email = account.get_email();
    Command::new("git")
        .args(&["config", "--global", "user.name", format!("\"{}\"", new_name).as_str()])
        .spawn()
        .expect("Failed to execute 'git config --global user.name' command!");
    println!("Command 'git config --global user.name' has been executed successfully!");
    Command::new("git")
        .args(&["config", "--global", "user.email", format!("\"{}\"", new_email).as_str()])
        .spawn()
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
