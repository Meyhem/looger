#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod apimodels;
mod configuration;
mod formvalue;
mod store;

use apimodels::{QueryLogModel, RequestLogModel};
use config::{Config, File};
use configuration::ApplicationConfig;
use formvalue::Rfc3339DateTime;
use rocket::{http::Status, State};
use rocket_contrib::json::Json;
use std::collections::HashMap;

type LoggerMap = HashMap<String, sled::Db>;

#[get("/<logger>?<offset>&<limit>&<from>&<to>", format = "json")]
fn get_log(
    logger: String,
    offset: usize,
    limit: usize,
    from: Rfc3339DateTime,
    to: Rfc3339DateTime,
    loggers: State<LoggerMap>,
) -> Result<Json<Vec<QueryLogModel>>, Status> {
    if !loggers.contains_key(&logger) {
        return Err(Status::NotFound);
    }
    let l = loggers.get(&logger.to_ascii_lowercase()).unwrap();

    Ok(Json(
        store::query(l, from.into(), to.into(), offset, limit)
            .iter()
            .map(|l| QueryLogModel::from(l))
            .collect(),
    ))
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
            let id = store::StoredLog::format_log_identifier(gid);
            store::new_stored_log(id, &log.level, log.scope.clone(), log.message.clone())
        })
        .collect();

    store::store_batch(&logs_to_store, l).unwrap();

    Status::Ok
}

fn main() {
    let mut cfg = Config::new();
    cfg.merge(File::with_name("luger")).expect("Load config");

    let config: ApplicationConfig = cfg.try_into().unwrap();

    let mut loggers: LoggerMap = HashMap::new();

    for l in config.loggers {
        let name = l.name.to_ascii_lowercase();
        let db = sled::open(&name).expect("Open db");
        loggers.insert(name, db);
    }

    rocket::ignite()
        .manage(loggers)
        .mount("/", routes![get_log, append_log])
        .launch();
}
