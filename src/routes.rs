use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::vec;
use regex::Regex;

use rocket::{
    self,
    Config,
    fs::NamedFile,
    serde::json::Json,
    serde::Deserialize,
    serde::Serialize,
    Request,
    http::Method,
};

use rocket_dyn_templates::Template;


#[rocket::get("/")]
async fn index() -> Template {
    println!("{:?}", get_crimes());

    for crime in get_crimes().unwrap() {
       println!("{}", get_address(crime.lat, crime.long).await);
    }

    Template::render("index", rocket_dyn_templates::context!{crimes: get_crimes().unwrap()})
}


#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Crime {
    lat: f32,
    long: f32,
    desc: String,
    loc: String,
}

#[rocket::post("/report", data = "<data>")]
fn report(data: Json<Crime>) -> String {
    let crime: Crime = Crime{lat: data.lat, long: data.long, desc: data.desc.clone(), loc: "".to_string()};
    add_crime(crime);
    println!("{:?}", data);
    "200".to_string()
}

fn add_crime(crime: Crime) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("crimes.txt")
        .expect("could not open file");

    file.write_all(format!("\n{}, {}, {}", crime.lat, crime.long, crime.desc.clone()).as_bytes()).expect("error writing crime");
}

async fn get_address(lat: f32, long: f32) -> String {
    let token: &str = "pk.eyJ1Ijoid2FybXN0IiwiYSI6ImNsOWJydXhzNjNyN2EzdmxmZW00bDN0OTEifQ.4pmON-6Tdj9ca_T8_8D_dw";
    let url = "https://api.mapbox.com/geocoding/v5/mapbox.places/".to_string() + long.to_string().as_str() + "," + lat.to_string().as_str() + ".json?access_token=" + token;

    let resp = reqwest::get(url)
        .await.unwrap()
        .text()
        .await.unwrap();

    let addr = Regex::new(r#"address":"(.*?)","#).unwrap();
    let name = Regex::new(r#"place_name":"(.*?)","#).unwrap();
    for cap in addr.captures_iter(&resp) {
        return cap[1].to_string();
    }

    for cap in name.captures_iter(&resp) {
        return cap[1].to_string();
    }

    format!("lat:{}, long:{}", lat, long).to_string()
}


fn get_crimes() -> Result<Vec<Crime>, String> {
    let contents = match read_to_string("crimes.txt") {
        Ok(n) => {n},
        Err(_) => {return Err("error reading file".to_string());},
    };
    
    let mut crimes: Vec<Crime> = vec![];

    for line in contents.split("\n") {
        let vals: Vec<&str> = line.split(",").collect();

        if vals.len() != 3 {
            continue;
        }

        let crime = Crime{lat: vals[0].trim().parse().unwrap(), long: vals[1].trim().parse().unwrap(), desc: vals[2].to_string(), loc: "".to_string()};
        crimes.push(crime);
    }

    Ok(crimes)
}

pub fn start_api() {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(Config::from(Config::figment()).workers)
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = rocket::build()
            .mount("/", rocket::routes![index, get_file, report])
            .attach(Template::fairing())
            //.manage()
            .launch()
            .await;
        });
}