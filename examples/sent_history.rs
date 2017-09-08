//! Envs:
//! "MF_INVOICE_ACCESS_TOKEN" -- access token
extern crate moneyforward_invoice_api as mf;
extern crate env_logger;
extern crate native_tls;

use mf::Client;
use std::env;

fn main() {
    env_logger::init().unwrap();

    let token = env::var("MF_INVOICE_ACCESS_TOKEN").unwrap();
    let mut client = Client::new(token).unwrap();

    let history = client.sent_history(1, 100);
    println!("{:#?}", history);
}
