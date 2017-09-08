//! Envs:
//! "MF_INVOICE_ACCESS_TOKEN" -- access token
extern crate moneyforward_invoice_api as mf;
extern crate env_logger;
extern crate native_tls;

use mf::{Client, NewItem, UpdateItem};
use std::env;

fn main() {
    env_logger::init().unwrap();

    let token = env::var("MF_INVOICE_ACCESS_TOKEN").unwrap();
    let mut client = Client::new(token).unwrap();

    let item = client
        .create_item(NewItem {
            name: "サンプル商品".into(),
            excise: Some(true),
            unit_price: Some(100),
            ..Default::default()
        })
        .unwrap()
        .unwrap();

    println!("created item: {:#?}", item);

    let item = client
        .update_item(
            &item.id,
            UpdateItem {
                name: Some("更新商品名".into()),
                ..Default::default()
            },
        )
        .unwrap()
        .unwrap();

    println!("updated item: {:#?}", item);

    let item = client.get_item(&item.id).unwrap().unwrap();

    println!("got item: {:#?}", item);

    let items = client.list_items().unwrap().unwrap();

    println!("list meta: {:#?}", items.meta);

    println!("items: {:#?}", items.items);

    client.delete_item(&item.id).unwrap().unwrap();

}
