//! Envs:
//! "MF_INVOICE_ACCESS_TOKEN" -- access token
extern crate moneyforward_invoice_api as mf;
extern crate env_logger;
extern crate native_tls;
extern crate chrono;

use mf::{Client, NewBilling, NewBillingItem, NewPartner, UpdateBilling, UpdateBillingItem};
use std::env;
use chrono::NaiveDate;

fn main() {
    env_logger::init().unwrap();

    let token = env::var("MF_INVOICE_ACCESS_TOKEN").unwrap();
    let mut client = Client::new(token).unwrap();

    let partner = client
        .create_partner(NewPartner {
            name: "サンプル取引先".into(),
            ..Default::default()
        })
        .unwrap()
        .unwrap();

    let billing = client
        .create_billing(NewBilling {
            department_id: partner.departments[0].clone().id,
            billing_date: Some(NaiveDate::from_ymd(2017, 9, 30)),
            items: vec![NewBillingItem { ..Default::default() }],
            ..Default::default()
        })
        .unwrap()
        .unwrap();

    println!("created billing: {:#?}", billing);

    let billing = client
        .update_billing(
            &billing.id,
            UpdateBilling {
                memo: Some("更新しました".into()),
                items: vec![
                    UpdateBillingItem {
                        id: Some(billing.items[0].clone().id),
                        _destroy: true,
                        ..Default::default()
                    },
                    UpdateBillingItem {
                        name: Some("新しい品目".into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        )
        .unwrap()
        .unwrap();

    println!("updated billing: {:#?}", billing);

    let billing = client.get_billing(&billing.id).unwrap().unwrap();
    println!("got billing: {:#?}", billing);


    let billings = client.list_billings(1, 100).unwrap().unwrap();
    println!("list metadata: {:#?}", billings.meta);
    println!("list :{:#?}", billings.billings);

    let billings = client
        .search_billings(1, 100, "サンプル", None, None, None)
        .unwrap()
        .unwrap();
    println!("search metadata: {:#?}", billings.meta);
    println!("search result :{:#?}", billings.billings);

    client.delete_billing(&billing.id).unwrap().unwrap();
    println!("deleted the billing");
}
