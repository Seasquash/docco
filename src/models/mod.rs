use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub index: Vec<String>,
    pub formats: Vec<ConfigFormat>
}

#[derive(Deserialize, Debug)]
pub struct ConfigFormat {
    pub extension: String,
    pub start: String,
    pub end: String
}
