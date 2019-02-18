extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    account: Vec<AccountSection>,
}

impl Config {
    pub fn get_account_by_name(&self, name: &str) -> Option<&AccountSection> {
        for account in &self.account {
            if account.get_name() == name {
                return Some(account);
            }
        }
        None
    }
}

#[derive(Deserialize)]
pub struct AccountSection {
    name: Option<String>,
    email: Option<String>,
}

impl AccountSection {
    pub fn get_name(&self) -> &String {
        self.name.as_ref().expect("There is no name parameter specified in the config!")
    }

    pub fn get_email(&self) -> &String {
        self.email.as_ref().expect("There is no email parameter specified in the config!")
    }
}

pub fn init_config(mut file: File) -> Config {
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Failed to read config file content!");
    toml::from_str(file_content.as_str()).expect("Error occurred while processing config file!")
}
