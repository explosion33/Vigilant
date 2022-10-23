use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::vec;
use regex::Regex;

use twilio_async::{Twilio, TwilioRequest};

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
    println!("{:?}", get_users().unwrap());
    Template::render("index", rocket_dyn_templates::context!{crimes: get_crimes().await.unwrap()})
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    lat: f32,
    long: f32,
    num: i64,
}

#[rocket::post("/report", data = "<data>")]
async fn report(data: Json<Crime>) -> String {
    let mut crime: Crime = Crime{lat: data.lat, long: data.long, desc: data.desc.clone(), loc: "".to_string()};
    add_crime(&crime);
    println!("{:?}", data);

    crime.loc = get_address(crime.lat, crime.long).await;

    handle_notifications(crime).await;

    "200".to_string()
}

fn add_crime(crime: &Crime) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("crimes.txt")
        .expect("could not open file");

    file.write_all(format!("\n{}, {}, {}", crime.lat, crime.long, crime.desc.clone()).as_bytes()).expect("error writing crime");
}


async fn get_crimes() -> Result<Vec<Crime>, String> {
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

        let lat: f32 = vals[0].trim().parse().unwrap();
        let long: f32 = vals[1].trim().parse().unwrap();
        let desc: String = vals[2].to_string();
        let loc: String = get_address(lat, long).await;

        let crime = Crime{lat, long, desc, loc};
        crimes.push(crime);
    }

    Ok(crimes)
}

#[rocket::post("/register", data = "<data>")]
fn register(data: Json<User>) -> String {
    let user: User = User{num: data.num, lat: data.lat, long: data.long};
    add_user(user);
    println!("{:?}", data);
    "200".to_string()
}



fn add_user(user: User) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("users.txt")
        .expect("could not open file");

    file.write_all(format!("\n{}, {}, {}", user.num, user.lat, user.long).as_bytes()).expect("error writing user");
}

fn get_users() -> Result<Vec<User>, String> {
    let contents = match read_to_string("users.txt") {
        Ok(n) => {n},
        Err(_) => {return Err("error reading file".to_string());},
    };
    
    let mut users: Vec<User> = vec![];

    for line in contents.split("\n") {
        let vals: Vec<&str> = line.split(",").collect();

        if vals.len() != 3 {
            continue;
        }

        let num: i64 = vals[0].trim().parse().unwrap();
        let lat: f32 = vals[1].trim().parse().unwrap();
        let long: f32 = vals[2].trim().parse().unwrap();

        let user = User{num, lat, long};
        users.push(user);
    }

    Ok(users)
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

async fn handle_notifications(crime: Crime) {
    for user in get_users().unwrap() {
        if get_distance(&user, &crime) <= 6f32 {
            println!("sending to user {}", user.num);
            send_alert(&user, &crime).await;
        }
    }
}

fn get_distance(user: &User, crime: &Crime) -> f32{
    let earth_r_km = 6371.0 ;
    let lat1 = user.lat*3.1415926/180.0;
    let lon1 = user.long*3.1415926/180.0;
    let lat2 = crime.lat*3.1415926/180.0;
    let lon2 = crime.long*3.1415926/180.0;
    
    let l = (((lat2-lat1)/2.0).sin())*(((lat2-lat1)/2.0).sin());
    //println!("l: {}", l);
    let r = (lat1.cos()*lat2.cos())*(((lon2-lon1)/2.0).sin())*(((lon2-lon1)/2.0).sin());
    //println!("r: {}", r);
    let b = (l+r).sqrt();
    //println!("b: {}", b);
    let d_km = 2.0*earth_r_km*((b).asin());
    //println!("d_km: {}", d_km);
    let d_mile = 0.62137119*d_km;
    return d_mile;
}


async fn send_alert(user: &User, crime: &Crime) {
    let twilio = Twilio::new("AC9500dd8203f63e10eb400155ed142e1d", "ffc4e47ab697e99d441b9e659c42a87a").unwrap();
    
    let mut msg = String::new();
    msg += "Alert! A crime has been reported within 6 miles of you're location\n";
    msg += "Location | ";
    msg += crime.loc.as_str();
    msg += "\nDescription | ";
    msg += crime.desc.as_str();

    println!("{}", ("+".to_string() + user.num.to_string().as_str()).as_str());

    twilio.send_msg("+19704272763", ("+".to_string() + user.num.to_string().as_str()).as_str(), msg.as_str()).run().await.unwrap();
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
            .mount("/", rocket::routes![index, get_file, report, register])
            .attach(Template::fairing())
            //.manage()
            .launch()
            .await;
        });
}