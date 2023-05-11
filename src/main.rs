mod campaign;
mod config;

fn main() {
    let config = config::Config::from_file();

    println!("{:?}", config)
}
