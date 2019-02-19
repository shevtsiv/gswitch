#[macro_use]
extern crate serde_derive;

mod config;
mod gitutils;

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
    gitutils::set_git_name(new_name);
    gitutils::set_git_email(new_email);
    let confirm_name = gitutils::get_git_name();
    let confirm_email = gitutils::get_git_email();
    println!("Your new credentials: ");
    println!("Name: {}", confirm_name);
    println!("Email: {}", confirm_email);
}
