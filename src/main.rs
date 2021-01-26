#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod loogerconfig;
mod models;
use config::{Config, File};
use loogerconfig::ApplicationConfig;
use models::RequestAppendLog;
use rocket::State;
use rocket_contrib::json::Json;
use sled::{Db, IVec};
use std::collections::HashMap;
use std::convert::TryInto;

type LoggerMap = HashMap<String, sled::Db>;

#[get("/")]
fn index(db: State<Db>) -> &'static str {
    let key = IVec::from("views");
    let stored = match db.get(&key).unwrap() {
        Some(v) => v,
        None => IVec::from(&[0u8; 4]),
    };

    let parsed: u32 = u32::from_be_bytes(stored.as_ref().try_into().unwrap());
    println!("Views: {}!", parsed);

    db.insert(&key, IVec::from(&(parsed + 1).to_be_bytes()))
        .unwrap();
    "Hello, world!"
}

#[post("/", format = "json", data = "<body>")]
fn append_log(
    // logger: String,
    body: Json<RequestAppendLog>,
    loggers: State<LoggerMap>,
) {
    println!("{:?}", body.timestamp.timestamp_millis());
}

fn main() {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("looger")).expect("Load config");

    let config: ApplicationConfig = cfg.try_into().unwrap();

    let mut loggers: LoggerMap = HashMap::new();

    for l in config.loggers {
        let name = l.name;
        let db = sled::open(&name).expect("Open db");
        loggers.insert(name, db);
    }

    rocket::ignite()
        .manage(loggers)
        .mount("/", routes![index, append_log])
        .launch();
}
