use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::{env, fmt};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub categories: HashMap<String, HashSet<String>>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = env::var("HOME")
            .map(PathBuf::from)
            .or_else(|_| env::var("USERPROFILE").map(PathBuf::from))?;

        let path = config_dir.join(".config/dl_sort/config.yaml");

        if !path.exists() {
            return Err(format!("Config file not found: {:?}", path).into());
        }

        let file_sring = fs::read_to_string(&path)?;
        let ret: Config = serde_yaml::from_str(&file_sring)?;

        Ok(ret)
    }
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        let config_yaml: &str = r#"categories:
                                      Audios:
                                        - mp3
                                        - wav
                                        - flac
                                      Compressed:
                                        - zip
                                        - rar
                                        - tar
                                      Developments:
                                        - rs
                                        - js
                                        - java
                                        - html
                                      PDFs:
                                        - pdf
                                        - txt
                                        - docx
                                      Photos:
                                        - jpg
                                        - png
                                        - jpeg
                                        - webp
                                      Videos:
                                        - mp4
                                        - mov
                                        - mkv
                                    "#;
        let ret: Config = serde_yaml::from_str(config_yaml).unwrap();
        Ok(ret)
    }
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(yaml) => write!(f, "{}", yaml),
            Err(_) => Err(fmt::Error),
        }
    }
}
