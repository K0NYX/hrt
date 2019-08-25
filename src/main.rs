#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

#[macro_use]
extern crate clap;
use clap::App;

mod qrz;
mod hamqth;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

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
    }
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
