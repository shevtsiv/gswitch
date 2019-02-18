extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    first: Option<AccountSection>,
    second: Option<AccountSection>,
    third: Option<AccountSection>,
}

impl Config {
    pub fn first_account(&self) -> &AccountSection {
        self.first.as_ref().expect("There is no first account section specified in the config!")
    }

    pub fn second_account(&self) -> &AccountSection {
        self.second.as_ref().expect("There is no second account section specified in the config!")
    }

    // The third account is not always required but possible, so we do not unwrap it here
    pub fn third_account(&self) -> Option<&AccountSection> {
        self.third.as_ref()
    }

    pub fn get_account_by_name(&self, name: &str) -> Option<&AccountSection> {
        if self.first_account().get_name() == name {
            Some(self.first_account())
        } else if self.second_account().get_name() == name {
            Some(self.second_account())
        } else {
            if self.third_account().is_some() && self.third_account().unwrap().get_name() == name {
                return self.third_account();
            }
            None
        }
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
