#[macro_use]
extern crate clap;
#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;

use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use clap::Arg;
use vercel_lambda::{error::VercelError, Handler, IntoResponse, lambda, Request, Response};
use vercel_lambda::http::StatusCode;

use crate::asns::*;
use crate::vercel::VercelAsnHandler;
use crate::webservice::*;

mod asns;
mod webservice;
mod vercel;

fn get_asns(db_url: &str) -> Result<ASNs, &'static str> {
    info!("Retrieving ASNs");
    let asns = ASNs::new(db_url);
    info!("ASNs loaded");
    asns
}

fn update_asns(asns_arc: &Arc<RwLock<Arc<ASNs>>>, db_url: &str) {
    let asns = match get_asns(db_url) {
        Ok(asns) => asns,
        Err(e) => {
            warn!("{}", e);
            return;
        }
    };
    *asns_arc.write().unwrap() = Arc::new(asns);
}

fn main() -> Result<(), Box<dyn Error>> {
    info!("Setting up db update");
    let db_url = "https://iptoasn.com/data/ip2asn-combined.tsv.gz";
    let asns = get_asns(&db_url).expect("Unable to load the initial database");
    let asns_arc = Arc::new(RwLock::new(Arc::new(asns)));
    let asns_arc_copy = asns_arc.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3600));
        update_asns(&asns_arc_copy, &db_url);
    });

    info!("Starting the lambda");
    Ok(lambda!(VercelAsnHandler { asns_arc }))
}
