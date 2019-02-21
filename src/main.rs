#[macro_use]
extern crate serde_derive;

mod config;
mod gitutils;

use std::fs::{File, copy, read_dir};
use std::env;
use crate::gitutils::{get_git_name, set_git_name, get_git_email, set_git_email};

fn main() {
    let config_file = File::open("settings.toml").expect("Failed to open config file!");
    let config = config::init_config(config_file);
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print_help();
        return;
    }
    let account = if args.len() == 1 {
        config.get_next_after(get_git_name().as_str())
    } else {
        let first_arg = args.get(1).unwrap();
        if first_arg == "--help" {
            print_help();
            return;
        }
        config.get_account_by_name(first_arg)
    };
    if account.is_none() {
        println!("There is no Git account with such name!");
        return;
    }
    let account = account.unwrap();
    let new_name = account.get_name();
    let new_email = account.get_email();
    for entry in read_dir(account.get_ssh_path()).unwrap() {
        let entry = entry.unwrap();
        let from = entry.path();
        let mut to = config.get_ssh_global_path();
        to.push_str(entry.file_name().to_str().unwrap());
        copy(from, to).expect("Error occurred while copying ssh keys!");
    }
    set_git_name(new_name);
    set_git_email(new_email);
    let confirm_name = get_git_name();
    let confirm_email = get_git_email();
    println!("Your new credentials: ");
    println!("Name: {}", confirm_name);
    println!("Email: {}", confirm_email);
}

fn print_help() {
    println!("Usage: ");
    println!("gswitch <username> - Switch to Git account with name 'username'");
    println!("gswitch            - Switch to the next Git account listed in the settings.toml");
    println!("gswitch --help     - Print help message");
}
