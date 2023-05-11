use campaign::CampaignInfo;

mod campaign;
mod config;

fn main() {
    let config = config::Config::from_file();

    let campaign_info = CampaignInfo::from_file(config).expect("Couldn't open campaign info file");
    println!("{:?}", campaign_info)
}
