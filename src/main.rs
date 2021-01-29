#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod apimodels;
mod configuration;
mod store;

use apimodels::RequestLogModel;
use config::{Config, File};
use configuration::ApplicationConfig;
use rocket::{http::Status, State};
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

#[post("/<logger>", format = "json", data = "<body>")]
fn append_log(
    logger: String,
    body: Json<Vec<RequestLogModel>>,
    loggers: State<LoggerMap>,
) -> Status {
    if !loggers.contains_key(&logger) {
        return Status::NotFound;
    }

    let l = loggers.get(&logger.to_ascii_lowercase()).unwrap();

    let logs_to_store: Vec<store::StoredLog> = body
        .iter()
        .map(|log| {
            let gid = l.generate_id().unwrap();
            let id = store::format_log_indetifier(gid);
            println!("{:?}", id);
            store::new_stored_log(id, &log.level, log.scope.clone(), log.message.clone())
        })
        .collect();

    store::store_batch(&logs_to_store, l).unwrap();

    Status::Ok
}

fn main() {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("looger")).expect("Load config");

    let config: ApplicationConfig = cfg.try_into().unwrap();

    let mut loggers: LoggerMap = HashMap::new();

    for l in config.loggers {
        let name = l.name.to_ascii_lowercase();
        let db = sled::open(&name).expect("Open db");
        loggers.insert(name, db);
    }

    rocket::ignite()
        .manage(loggers)
        .mount("/", routes![index, append_log])
        .launch();
}
