use keyring::Entry;
use std::{fs, io::ErrorKind, path::PathBuf, process};

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub github: Option<GitHubConfig>,
    pub jira: Option<JiraConfig>,
}

#[derive(Debug)]
pub struct GitHubConfig {
    pub host: String,
    pub username: String,
    pub api_token: String,
}

#[derive(Debug)]
pub struct JiraConfig {
    pub email: String,
    pub host: String,
    pub api_token: String,
}

impl Config {
    pub fn load() -> Config {
        let toml_config = TomlConfig::from_file().expect("TODO: handle config errros");
        Config {
            github: toml_config.github.map(|v| {
                let api_token = get_api_token_form_keyring(ApiService::GitHub, &v.username);
                GitHubConfig {
                    host: v.host,
                    username: v.username,
                    api_token,
                }
            }),
            jira: toml_config.jira.map(|v| {
                let api_token = get_api_token_form_keyring(ApiService::Jira, &v.email);
                JiraConfig {
                    host: v.host,
                    email: v.email,
                    api_token,
                }
            }),
        }
    }
}

/// Usernames which are found in the TOML file
/// These are the services which need to be authenticated for
pub struct ConfiguredServices {
    pub github: Option<String>,
    pub jira: Option<String>,
}

impl ConfiguredServices {
    pub fn from_file() -> ConfiguredServices {
        let config = TomlConfig::from_file().expect("TODO: handle config errors");
        ConfiguredServices {
            github: config.github.map(|v| v.username),
            jira: config.jira.map(|v| v.email),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file does not exist at path {0}")]
    FileNotFound(PathBuf),
}

#[derive(Deserialize)]
pub struct TomlConfig {
    github: Option<TomlGitHubConfig>,
    jira: Option<TomlJiraConfig>,
}

#[derive(Deserialize)]
struct TomlGitHubConfig {
    host: String,
    username: String,
}

#[derive(Deserialize)]
struct TomlJiraConfig {
    host: String,
    email: String,
}

impl TomlConfig {
    pub fn from_file() -> Result<TomlConfig, ConfigError> {
        let config_file_path = get_base_directories().in_config_dir("cirro.toml");
        let config_file_content =
            fs::read_to_string(&config_file_path).map_err(|e| match e.kind() {
                ErrorKind::NotFound | ErrorKind::PermissionDenied => {
                    ConfigError::FileNotFound(config_file_path)
                }
                _ => {
                    eprintln!(
                        "An unexpected error occured whilst reading config file: {}.\nError: {}",
                        e,
                        config_file_path.display()
                    );
                    process::exit(1);
                }
            })?;
        Ok(toml::from_str(config_file_content.as_str()).unwrap())
    }
}

fn get_base_directories() -> impl AppStrategy {
    let app_strategy_args = AppStrategyArgs {
        top_level_domain: "dev".to_owned(),
        author: "Dan Jones".to_owned(),
        app_name: "cirro".to_owned(),
    };
    match choose_app_strategy(app_strategy_args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Could not find home directory: {}", e);
            process::exit(1);
        }
    }
}

#[derive(Debug)]
pub enum ApiService {
    GitHub,
    Jira,
}

impl ApiService {
    fn keryring_name(&self) -> &'static str {
        match self {
            ApiService::GitHub => "cirro-github",
            ApiService::Jira => "cirro-jira",
        }
    }
}

fn get_api_token_form_keyring(service: ApiService, username: &str) -> String {
    let cred = match Entry::new(&service.keryring_name(), username) {
        Ok(v) => v,
        Err(e) => match e {
            // keyring::Error::PlatformFailure(error) => ${1:todo!()},
            // keyring::Error::NoStorageAccess(error) => ${2:todo!()},
            // keyring::Error::NoEntry => ${3:todo!()},
            // keyring::Error::BadEncoding(vec) => ${4:todo!()},
            // keyring::Error::TooLong(_, _) => ${5:todo!()},
            // keyring::Error::Invalid(_, _) => ${6:todo!()},
            // keyring::Error::Ambiguous(vec) => ${7:todo!()},
            _ => {
                eprint!(
                    "Unexpected error occured whilst getting the github api token: {}",
                    e
                );
                std::process::exit(1);
            }
        },
    };
    match cred.get_password() {
        Ok(v) => v,
        Err(e) => match e {
            keyring::Error::NoEntry => todo!("Handle no entry"),
            keyring::Error::Ambiguous(_vec) => todo!("handle ambiguity"),
            _ => {
                eprint!(
                    "Unexpected error occured whilst getting the {:?} api token: {}",
                    service, e
                );
                std::process::exit(1);
            }
        },
    }
}

pub fn store_api_token_in_keyring(service: ApiService, username: &str, api_token: &str) {
    let cred = match Entry::new(&service.keryring_name(), username) {
        Ok(v) => v,
        Err(e) => match e {
            // keyring::Error::PlatformFailure(error) => ${1:todo!()},
            // keyring::Error::NoStorageAccess(error) => ${2:todo!()},
            // keyring::Error::NoEntry => ${3:todo!()},
            // keyring::Error::BadEncoding(vec) => ${4:todo!()},
            // keyring::Error::TooLong(_, _) => ${5:todo!()},
            // keyring::Error::Invalid(_, _) => ${6:todo!()},
            // keyring::Error::Ambiguous(vec) => ${7:todo!()},
            _ => {
                eprint!(
                    "Unexpected error occured whilst getting the github api token: {}",
                    e
                );
                std::process::exit(1);
            }
        },
    };
    cred.set_password(api_token)
        .expect("Error setting api token in keyring");
}
