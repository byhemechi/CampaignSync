use serde_env::from_env;
use std::fs;

#[derive(Debug, serde::Deserialize)]
pub struct Env {
    #[cfg(windows)]
    #[serde(rename = "userprofile")]
    pub home: String,
    #[cfg(not(windows))]
    pub home: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub install_dir: String,
    pub app_data_dir: String,
    pub campaign_name: String,
}

impl Config {
    pub fn default<'a>() -> Config {
        let env: Env = from_env().expect("Couldn't deserialise environment");
        Config {
            install_dir: r#"C:\Program Files (x86)\Steam\steamapps\common\Beat Saber"#.to_string(),
            campaign_name: "Acc Champ Community Campaign".to_string(),
            app_data_dir: format!(
                r#"{}\AppData\LocalLow\Hyperbolic Magnetism\Beat Saber"#,
                env.home
            ),
        }
    }

    // TODO: Clean this up
    pub fn from_file() -> Config {
        match fs::read_to_string("./campaignsync.ron") {
            Err(e) => {
                println!("{:?}", e);
                let new_config = Config::default();
                match fs::write(
                    "./campaignsync.ron",
                    ron::ser::to_string_pretty(
                        &new_config,
                        ron::ser::PrettyConfig::struct_names(
                            ron::ser::PrettyConfig::default(),
                            true,
                        ),
                    )
                    .expect("Couldn't serialise default config"),
                ) {
                    Err(_) => {
                        println!("Couldn't write default config file")
                    }
                    _ => {}
                };
                new_config
            }
            Ok(d) => match ron::from_str(&d) {
                Ok(cfg) => cfg,
                Err(_) => {
                    println!("Failed to parse config file, using default");
                    Config::default()
                }
            },
        }
    }
}
