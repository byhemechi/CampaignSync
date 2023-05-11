use std::{error::Error, fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapPosition {
    child_nodes: Vec<i32>,
    x: f64,
    y: f64,
    scale: f64,
    #[serde(default)]
    letter_portion: String,
    number_portion: i32,
    node_outline_location: String,
    node_background_location: String,
    node_default_color: String,
    node_highlight_color: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LightColor {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignInfo {
    name: String,
    desc: String,
    big_desc: String,
    all_unlocked: bool,
    map_positions: Vec<MapPosition>,
    unlock_gate: Vec<serde_json::Value>,
    map_height: i32,
    background_alpha: f64,
    light_color: LightColor,
    custom_mission_leaderboard: String,
}

impl CampaignInfo {
    pub fn from_file(config: Config) -> Result<CampaignInfo, Box<dyn Error>> {
        let file = format!(
            r#"{}\CustomCampaigns\{}\info.json"#,
            config.install_dir, config.campaign_name
        );
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }
}
