extern crate reqwest;

use serde_xml_rs::from_str;

#[derive(Debug, Deserialize)]
struct Session {
    #[serde(default)]
    session_id: String
}

impl Default for Session {
    fn default() -> Session {
        Session {
            session_id: "".to_string()
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

pub fn session(username: &str, password: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let query_resp = client.get("https://www.hamqth.com/xml.php")
        .query(&[("u", username), ("p", password)])
        .send()?
        .text()?;

    let hqth: HamQTH = from_str(&query_resp).unwrap();

    Ok(hqth.session.session_id)
}

pub fn query(session_id: &str, callsign: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let query_resp = client.get("https://www.hamqth.com/xml.php")
        .query(&[("id", session_id), ("callsign", callsign), ("prg", "hrt")])
        .send()?
        .text()?;

    let hqth: HamQTH = from_str(&query_resp).unwrap();
    println!("\n{} (HamQTH)", hqth.search.callsign);
    println!("  Name: {}", hqth.search.adr_name);
    println!("  Location: {}, {}, {}", hqth.search.adr_city, hqth.search.us_state, hqth.search.adr_country);
    Ok(())
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