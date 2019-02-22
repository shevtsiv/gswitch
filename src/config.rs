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

    pub fn get_ssh_global_path(&self) -> Option<String> {
        let ssh_global_location = self.ssh_global_path.clone();
        if ssh_global_location.is_none() {
            return None;
        }
        let mut ssh_global_location = ssh_global_location.unwrap();
        if ssh_global_location.is_empty() {
            return None;
        }
        let last_symbol = ssh_global_location.chars().last().unwrap();
        if last_symbol != '/' && last_symbol != '\\' {
            ssh_global_location.push_str("/");
            return Some(ssh_global_location);
        }
        Some(ssh_global_location)
    }
}

#[derive(Deserialize)]
pub struct AccountSection<'a> {
    #[serde(borrow)]
    name: &'a str,
    #[serde(borrow)]
    email: &'a str,
    #[serde(borrow)]
    ssh_path: Option<&'a str>,
}

impl<'a> AccountSection<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_email(&self) -> &str {
        self.email
    }

    pub fn get_ssh_path(&self) -> Option<&str> {
        if self.ssh_path.is_none() {
            return None;
        }
        if self.ssh_path.unwrap().is_empty() {
            return None;
        }
        self.ssh_path
    }
}

pub fn init_config(config_file_content: &str) -> Config {
    let config: Config = toml::from_str(config_file_content).expect("Error occurred while processing config file!");
    if config.get_accounts_amount() < 2 {
        panic!("There are not enough Git accounts in the config file!");
    }
    config
}
