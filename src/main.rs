#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

#[macro_use]
extern crate clap;
use clap::App;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod qrz;
mod hamqth;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };
    let config_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.toml");
    if !Path::new(&config_path).exists() {
        create_config(&config_path);
    }

    if let Some(sub_matches) = matches.subcommand_matches("call") {
        if let Some(callsign) = sub_matches.value_of("CALLSIGN") {
            let mut source = "qrz".to_string();
            if let Some(s) = sub_matches.value_of("source") {
                source = s.to_string();
            }

            if source == "hamqth" {
                hamqth_call(&callsign, "call");
            } else {
                qrz_call(&callsign, "call");
            }
            
        }
    } else if let Some(sub_matches) = matches.subcommand_matches("dxcc") {
        if let Some(entity) = sub_matches.value_of("ENTITY") {
            let mut source = "qrz".to_string();
            if let Some(s) = sub_matches.value_of("source") {
                source = s.to_string();
            }

            if source == "hamqth" {
                hamqth_call(&entity, "dxcc");
            } else {
                qrz_call(&entity, "dxcc");
            }
            
        }
    } else if let Some(sub_matches) = matches.subcommand_matches("init") {
        if !Path::new(&config_path).exists() {
            create_config(&config_path);
        }
        println!("hrt config file location at {}", config_path);
        println!("Open the file with an editor and assign config values to run commands.");
    } else {
        App::from_yaml(yaml).print_help();
    }
}

fn create_config(config_path: &str) -> std::io::Result<()> {
    let config_str = r###"
# QRZ Login - callsign lookup source
qrz_callsign = ""
qrz_password = ""
# HamQTH Login - callsign lookup source
hamqth_callsign = ""
hamqth_password = ""
# QTH Location (use degrees)
latitude = ""
longitude = ""
# Dark Sky API Key - weather
darksky_api_key = ""
"###;

    let mut file = File::create(&config_path)?;
    file.write_all(config_str.as_bytes())?;

    Ok(())
}

fn qrz_call(callsign: &str, query_type: &str) {
    if query_type == "call" {
        let _query = qrz::query(&callsign);
    } else if query_type == "dxcc" {
        let _query = qrz::dxcc(&callsign);
    }
}

fn hamqth_call(callsign: &str, query_type: &str) {
    if query_type == "call" {
        let _query = hamqth::query(&callsign);
    } else if query_type == "dxcc" {
        let _query = hamqth::dxcc(&callsign);
    }
}
