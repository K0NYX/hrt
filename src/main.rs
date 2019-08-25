extern crate config;
extern crate dirs;

#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

#[macro_use]
extern crate clap;
use clap::App;

mod qrz;
mod hamqth;

use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };

    let config_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.toml");
    let mut settings = config::Config::default();
    settings.merge(config::File::from(Path::new(&config_path))).unwrap();

    if let Some(sub_matches) = matches.subcommand_matches("call") {
        if let Some(callsign) = sub_matches.value_of("CALLSIGN") {
            let mut source = settings.get_str("cs_default").unwrap();

            if let Some(s) = sub_matches.value_of("source") {
                source = s.to_string();
            }

            if source == "hamqth" {
                let username = settings.get_str("hamqth_callsign").unwrap();
                let password = settings.get_str("hamqth_password").unwrap();
                hamqth_call(&username, &password, &callsign);
            } else {
                let username = settings.get_str("qrz_callsign").unwrap();
                let password = settings.get_str("qrz_password").unwrap();
                qrz_call(&username, &password, &callsign);
            }
            
        }
    }
}

fn qrz_call(username: &str, password: &str, callsign: &str) {
    let key = match qrz::session(&username, &password) {
        Ok(k) => k,
        Err(_e) => panic!("error")
    };
    let _query = qrz::query(&key, &callsign);
}

fn hamqth_call(username: &str, password: &str, callsign: &str) {
    let key = match hamqth::session(&username, &password) {
        Ok(k) => k,
        Err(_e) => panic!("error")
    };
    let _query = hamqth::query(&key, &callsign);
}