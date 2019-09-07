extern crate config;
extern crate dirs;
extern crate reqwest;

use serde_xml_rs::from_str;
use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Session {
    #[serde(default)]
    session_id: String,
    #[serde(default)]
    error: String
}

impl Default for Session {
    fn default() -> Session {
        Session {
            session_id: "".to_string(),
            error: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Search {
    #[serde(default)]
    callsign: String,
    #[serde(default)]
    nick: String,
    #[serde(default)]
    qth: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    adif: String,
    #[serde(default)]
    itu: String,
    #[serde(rename="CQ", default)]
    cq: String,
    #[serde(default)]
    grid: String,
    #[serde(default)]
    adr_name: String,
    #[serde(default)]
    adr_street1: String,
    #[serde(default)]
    adr_street2: String,
    #[serde(default)]
    adr_street3: String,
    #[serde(default)]
    adr_city: String,
    #[serde(default)]
    adr_zip: String,
    #[serde(default)]
    adr_country: String,
    #[serde(default)]
    adr_adif: String,
    #[serde(default)]
    district: String,
    #[serde(default)]
    us_state: String,
    #[serde(default)]
    us_county: String,
    #[serde(default)]
    oblast: String,
    #[serde(default)]
    dok: String,
    #[serde(default)]
    iota: String,
    #[serde(default)]
    qsl_via: String,
    #[serde(default)]
    lotw: String,
    #[serde(default)]
    eqsl: String,
    #[serde(default)]
    qsl: String,
    #[serde(default)]
    qsldirect: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    jabber: String,
    #[serde(default)]
    icq: String,
    #[serde(default)]
    msn: String,
    #[serde(default)]
    skype: String,
    #[serde(default)]
    birth_year: String,
    #[serde(default)]
    lic_year: String,
    #[serde(default)]
    picture: String,
    #[serde(default)]
    latitude: String,
    #[serde(default)]
    longitude: String,
    #[serde(default)]
    continent: String,
    #[serde(default)]
    utc_offset: String,
    #[serde(default)]
    facebook: String,
    #[serde(default)]
    twitter: String,
    #[serde(default)]
    gplus: String,
    #[serde(default)]
    youtube: String,
    #[serde(default)]
    linkedin: String,
    #[serde(default)]
    flicker: String,
    #[serde(default)]
    vimeo: String
}

impl Default for Search {
    fn default() -> Search {
        Search {
            callsign: "".to_string(),
            nick: "".to_string(),
            qth: "".to_string(),
            country: "".to_string(),
            adif: "".to_string(),
            itu: "".to_string(),
            cq: "".to_string(),
            grid: "".to_string(),
            adr_name: "".to_string(),
            adr_street1: "".to_string(),
            adr_street2: "".to_string(),
            adr_street3: "".to_string(),
            adr_city: "".to_string(),
            adr_zip: "".to_string(),
            adr_country: "".to_string(),
            adr_adif: "".to_string(),
            district: "".to_string(),
            us_state: "".to_string(),
            us_county: "".to_string(),
            oblast: "".to_string(),
            dok: "".to_string(),
            iota: "".to_string(),
            qsl_via: "".to_string(),
            lotw: "".to_string(),
            eqsl: "".to_string(),
            qsl: "".to_string(),
            qsldirect: "".to_string(),
            email: "".to_string(),
            jabber: "".to_string(),
            icq: "".to_string(),
            msn: "".to_string(),
            skype: "".to_string(),
            birth_year: "".to_string(),
            lic_year: "".to_string(),
            picture: "".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            continent: "".to_string(),
            utc_offset: "".to_string(),
            facebook: "".to_string(),
            twitter: "".to_string(),
            gplus: "".to_string(),
            youtube: "".to_string(),
            linkedin: "".to_string(),
            flicker: "".to_string(),
            vimeo: "".to_string()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Dxcc {
    #[serde(default)]
    callsign: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    details: String,
    #[serde(default)]
    continent: String,
    #[serde(default)]
    utc: String,
    #[serde(default)]
    waz: String,
    #[serde(default)]
    itu: String,
    #[serde(default)]
    lat: String,
    #[serde(default)]
    lon: String,
    #[serde(default)]
    adif: String
}

impl Default for Dxcc {
    fn default() -> Dxcc {
        Dxcc {
            callsign: "".to_string(),
            name: "".to_string(),
            details: "".to_string(),
            continent: "".to_string(),
            utc: "".to_string(),
            waz: "".to_string(),
            itu: "".to_string(),
            lat: "".to_string(),
            lon: "".to_string(),
            adif: "".to_string()
        }
    }
}

#[derive(Debug, Deserialize)]
struct HamQTH {
    version: String,
    #[serde(default)]
    session: Session,
    #[serde(default)]
    search: Search,
    #[serde(default)]
    dxcc: Dxcc
}

pub fn session() -> Result<(), reqwest::Error> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };

    let config_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.toml");
    let mut settings = config::Config::default();
    settings.merge(config::File::from(Path::new(&config_path))).unwrap();
    let username = settings.get_str("hamqth_callsign").unwrap();
    let password = settings.get_str("hamqth_password").unwrap();

    let client = reqwest::Client::new();
    let query_resp = client.get("https://www.hamqth.com/xml.php")
        .query(&[("u", username), ("p", password)])
        .send()?
        .text()?;

    let hqth: HamQTH = from_str(&query_resp).unwrap();

    if hqth.session.error != ""
    {
        panic!("ERROR! {}", hqth.session.error);
    } else {
        set_session(hqth.session.session_id);
        Ok(())
    }
}

pub fn set_session(key: String) -> std::io::Result<()> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };
    let session_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.hamqth");

    fs::write(&session_path, key)?;
    Ok(())
}

pub fn get_session() -> String {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("error"),
    };
    let session_path = format!("{}{}", home_dir.to_string_lossy(), "/.hrt.hamqth");

    if !Path::new(&session_path).exists() {
        let _s = match session() {
            Ok(k) => k,
            Err(_e) => panic!("error")
        };
    }

    let mut file = File::open(&session_path).expect("Unable to open the file");
    let mut session_id = String::new();
    file.read_to_string(&mut session_id).expect("Unable to read the file");

    return session_id;
}

pub fn query(callsign: &str) -> Result<(), reqwest::Error> {
    let session_id = get_session();
    
    let client = reqwest::Client::new();

    let query_resp = client.get("https://www.hamqth.com/xml.php")
        .query(&[("id", session_id), ("callsign", callsign.to_string()), ("prg", "hrt".to_string())])
        .send()?
        .text()?;

    let hqth: HamQTH = from_str(&query_resp).unwrap();
    
    if hqth.session.error == "Session does not exist or expired" {
        let _s = match session() {
            Ok(k) => k,
            Err(_e) => panic!("error")
        };
        query(callsign)?;
        Ok(())
    } else if hqth.session.error != "" {
        panic!("ERROR! {}", hqth.session.error);
    } else {
        println!("\n{} (HamQTH)", hqth.search.callsign.to_uppercase());
        println!("  Name: {}", hqth.search.adr_name);
        if !hqth.search.email.is_empty() {
            println!("  Email: {}", hqth.search.email);
        }
        if !hqth.search.adr_street1.is_empty() {
            println!("  Address: {}", hqth.search.adr_street1);
        }
        if !hqth.search.adr_city.is_empty() {
            if !hqth.search.us_state.is_empty() {
                println!("  Location: {}, {} {}", hqth.search.adr_city, hqth.search.us_state, hqth.search.adr_zip);
            }
            else {
                println!("  Location: {}", hqth.search.adr_city);
            }
        }
        println!("  Country: {}", hqth.search.adr_country);
        Ok(())
    }
}

pub fn dxcc(entity: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let query_resp = client.get("https://www.hamqth.com/dxcc.php")
        .query(&[("callsign", entity)])
        .send()?
        .text()?;

    let hqth: HamQTH = from_str(&query_resp).unwrap();
    println!("\n{} (HamQTH)", hqth.dxcc.adif);
    println!("  Name: {}", hqth.dxcc.name);
    println!("  ITU: {}", hqth.dxcc.itu);
    println!("  UTC: {}", hqth.dxcc.utc);
    println!("  Details: {}", hqth.dxcc.details);
    Ok(())
}