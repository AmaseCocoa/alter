use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::{fs::File, io};

use dirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Account {
    #[serde(rename = "details")]
    AccountDetails {
        id: Uuid,
        slug: String,
        host: String,
        username: String,
        email: String,
        #[serde(rename = "withCred")]
        with_cred: bool,
        #[serde(rename = "signingkey")]
        signing_key: Option<String>,
    },
    #[serde(rename = "linked")]
    LinkedAccount {
        id: Uuid,
        slug: String,
        host: String,
        #[serde(rename = "linkTo")]
        link_to: Uuid,
        #[serde(rename = "withCred")]
        with_cred: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(rename = "signingkey")]
    pub signing_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub accounts: Vec<Account>,
    pub aliases: Vec<Alias>,
    #[serde(rename = "inUse")]
    pub in_use: Option<Uuid>,
}

impl Config {
    pub fn get_alias_by_id(&self, id: &Uuid) -> Option<&Alias> {
        self.aliases.iter().find(|alias| &alias.id == id)
    }

    pub fn is_in_use_uuid_valid(&self) -> bool {
        self.in_use.is_some()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = get_config_path()?;
        let mut file = File::create(path)?;
    
        let toml_string = toml::to_string(&self)?; 
        
        file.write_all(toml_string.as_bytes())?;
        
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, io::Error> {
    let mut path: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Home directory not found",
            ));
        }
    };
    path.push(".altercfg");
    Ok(path)
}

pub fn load_config() -> Result<Config, io::Error> {
    let path = get_config_path()?;

    match File::open(&path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut str = String::new();
            reader.read_to_string(&mut str).expect("cannot read string");
            match toml::from_str(&str) {
                Ok(config) => Ok(config),
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse config file: {}", e),
                )),
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let default_config = Config {
                accounts: vec![],
                aliases: vec![],
                in_use: None,
            };

            let toml_data = match toml::to_string_pretty(&default_config) {
                Ok(data) => data,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to serialize default config: {}", e),
                    ));
                }
            };

            let mut file = File::create(&path)?;

            file.write_all(toml_data.as_bytes())?;

            Ok(default_config)
        }
        Err(e) => Err(e),
    }
}
