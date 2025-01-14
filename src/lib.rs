use config::{store_api_token_in_keyring, Config, ConfiguredServices};
use rpassword;

pub mod cli_args;
pub mod config;

pub fn authenticate() {
    let config = ConfiguredServices::from_file();
    if config.github.is_some() {
        let username = config.github.unwrap();
        let api_token = rpassword::prompt_password(format!("GitHub API token for {username}: "))
            .expect("API token should be readable by program.");
        store_api_token_in_keyring(config::ApiService::GitHub, &username, &api_token);
    }

    if config.jira.is_some() {
        let username = config.jira.unwrap();
        let api_token = rpassword::prompt_password(format!("Jira API token for {username}: "))
            .expect("API token should be readable by program.");
        store_api_token_in_keyring(config::ApiService::Jira, &username, &api_token);
    }
}

pub fn run_tui() {
    let conf = Config::load();
    println!("Hello, world! {:?}", conf);
}
