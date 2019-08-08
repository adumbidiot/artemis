mod config;
mod unit;

use clap::{App, Arg};

use config::Config;
use std::path::Path;

const ARTEMIS_CONFIG: &str = r#"[Service]
name = "quarky"
start = "/home/pi/.cargo/bin/cargo +stable run --release"
update = "git pull"
"#;

fn main() {
    let matches = App::new("artemis")
        .author("adumbidiot <nathaniel.daniel23@outlook.com>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Load an artemis config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let artemis_config_path = Path::new(matches.value_of("config").unwrap_or("Artemis.toml"));
    if !(artemis_config_path.exists() && artemis_config_path.is_file()) {
		panic!("Unable to Load File");
	}
	
	let config_data = std::fs::read_to_string(&artemis_config_path).expect("Failed to read Config");

    let unit_dir = "/etc/systemd/system";
	//let unit_dir = "./";
	
    let artemis_config: Config = toml::from_str(&config_data).expect("Failed to parse");
    println!("Artemis Config:\n{:#?}\n", &artemis_config);

    let mut unit_file = artemis_config.to_unit();

    let unit_section = unit_file.get_section_mut("Unit").unwrap();
    unit_section.add("Wants", "network-online.target");
    unit_section.add("After", "network-online.target");

    let service_section = unit_file.get_section_mut("Service").unwrap();
    service_section.add("WorkingDirectory", "/home/pi/adumbidiot/quarky/");
    service_section.add("Environment", "\"RUSTC_WRAPPER=sccache\"");
    service_section.add("Restart", "always");
    service_section.add("User", "pi");
	
	let output = unit_file.output();
    println!("SystemD Unit:\n{}\n", &output);

    let unit_path = format!(
        "{}/{}.service",
        unit_dir,
        artemis_config.get_service().get_name()
    );
    println!("Saving to {}", unit_path);
	std::fs::write(unit_path, &output).expect("Failed to save file");
}
