use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoadedRegion {
    pub squares: Vec<(u32, u32)>,
    pub region_type: String,
    pub region_value: u32,
}

#[derive(Deserialize, Debug)]
pub struct LoadedGame {
    pub dominoes: Vec<(u32, u32)>,
    pub regions: Vec<LoadedRegion>,
}

pub fn load_games(filepath: &str) -> Vec<LoadedGame> {
    let games_content = fs::read_to_string(filepath).unwrap();
    serde_json::from_str(&games_content).unwrap()
}
