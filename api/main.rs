use std::error::Error;
use std::sync::{Arc, RwLock};
use vercel_lambda::lambda;

use crate::asns::Asns;
use crate::vercel::VercelAsnHandler;

mod asns;
mod vercel;
mod webservice;

async fn get_asns(db_url: &str) -> Result<Asns, Box<dyn Error>> {
    log::info!("Retrieving ASNs");
    let asns = Asns::new(db_url).await?;
    log::info!("ASNs loaded");
    Ok(asns)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    log::info!("Setting up db");
    let db_url = "https://iptoasn.com/data/ip2asn-combined.tsv.gz";
    let asns = get_asns(&db_url).await?;
    let asns_arc = Arc::new(RwLock::new(Arc::new(asns)));

    log::info!("Starting the lambda");
    Ok(lambda!(VercelAsnHandler { asns_arc }))
}
