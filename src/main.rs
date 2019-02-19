#[macro_use]
extern crate serde_derive;

mod config;
mod gitutils;

use std::fs::File;
use std::env;
use crate::gitutils::{get_git_name, set_git_name, get_git_email, set_git_email};

fn main() {
    let config_file = File::open("settings.toml").expect("Failed to open config file!");
    let config = config::init_config(config_file);
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: ");
        println!("gswitch <username> - Switch to Git account with name 'username'");
        println!("gswitch            - Switch to the next Git account listed in the settings.toml");
        return;
    }
    let account = {
        if args.len() == 1 {
            config.get_next_after(get_git_name().as_str())
        } else {
            config.get_account_by_name(args.get(1).unwrap())
        }
    };
    if account.is_none() {
        println!("There is no Git account with such name!");
        return;
    }
    let account = account.unwrap();
    let new_name = account.get_name();
    let new_email = account.get_email();
    set_git_name(new_name);
    set_git_email(new_email);
    let confirm_name = get_git_name();
    let confirm_email = get_git_email();
    println!("Your new credentials: ");
    println!("Name: {}", confirm_name);
    println!("Email: {}", confirm_email);
}
