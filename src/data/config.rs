
//use std::env;
use std::fs;

use super::super::constants;

use log;
use serde::{Serialize, Deserialize};
use serde_json::Error;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub site_domain: String,
    pub site_author: String,
    pub author_twitter: String,
    pub author_email: String,
    pub author_github_name: String,
    pub port: u32,
    pub host: String,
    pub db_file: String
}

impl Config {
    pub fn load() -> Self {
        Self::from_file(constants::DEFAULT_CONFIG_FILE).expect("Unable to load file ./config.json")
    }

    pub fn from_file(file_path: &str) -> Result<Self, &str> {
        let file_contents = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => panic!("Unable to read file {}", file_path)
        };
        let json_config : Result<Config, Error> = serde_json::from_str(&file_contents);
        match json_config {
            Ok(conf) => Ok(conf),
            Err(_) => {
                log::error!("Error parsing file {}",  file_path);
                Err("Parse error")
            }
        }
    }
}