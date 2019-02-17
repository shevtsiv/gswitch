extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    name: Option<String>,
    email: Option<String>,
}

impl Config {
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
