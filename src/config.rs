extern crate toml;

#[derive(Deserialize)]
pub struct Config<'a> {
    ssh_global_path: Option<String>,
    #[serde(borrow)]
    account: Vec<AccountSection<'a>>,
}

impl<'a> Config<'a> {
    pub fn get_account_by_name(&self, name: &str) -> Option<&AccountSection> {
        for account in &self.account {
            if account.get_name() == name {
                return Some(account);
            }
        }
        None
    }

    pub fn get_accounts_amount(&self) -> usize {
        self.account.len()
    }

    pub fn get_next_after(&self, name: &str) -> Option<&AccountSection> {
        let current_index = self.account.iter()
            .position(|elem| elem.get_name() == name)
            .expect("There is no Git account with such name!");
        let next_index = {
            if current_index == self.account.len() - 1 {
                0
            } else {
                current_index + 1
            }
        };
        self.account.get(next_index)
    }

    pub fn get_ssh_global_path(&self) -> String {
        let path = self.ssh_global_path.as_ref()
            .expect("There is no ssh_global_path parameter specified in the config!");
        let last_symbol = path.chars().last().unwrap();
        let mut new_path = path.clone();
        if last_symbol != '/' && last_symbol != '\\' {
            new_path.insert(path.len(), '/');
        }
        new_path
    }
}

#[derive(Deserialize)]
pub struct AccountSection<'a> {
    #[serde(borrow)]
    name: &'a str,
    #[serde(borrow)]
    email: &'a str,
    ssh_path: Option<String>,
}

impl<'a> AccountSection<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_email(&self) -> &str {
        self.email
    }

    pub fn get_ssh_path(&self) -> String {
        let path = self.ssh_path.as_ref().expect("There is no ssh_path parameter specified in the config!");
        let last_symbol = path.get(path.len()..path.len()).unwrap();
        let mut new_path = path.clone();
        if last_symbol != "/" && last_symbol != "\\" {
            new_path.insert(path.len(), '/');
        }
        new_path
    }
}

pub fn init_config(config_file_content: &str) -> Config {
    let config: Config = toml::from_str(config_file_content).expect("Error occurred while processing config file!");
    if config.get_accounts_amount() < 2 {
        panic!("There are not enough Git accounts in the config file!");
    }
    config
}
