//! Envs:
//! "MF_INVOICE_ACCESS_TOKEN" -- access token
extern crate moneyforward_invoice_api as mf;
extern crate env_logger;
extern crate native_tls;
extern crate oauth2;
extern crate url;

use mf::{NewPartner, UpdatePartner, UpdateDepartmentInfo};
use std::env;

fn main() {
    env_logger::init().unwrap();

    let token = env::var("MF_INVOICE_ACCESS_TOKEN").unwrap();
    let mut client = mf::Client::new(token).unwrap();
    // 取引先を作成
    let partner = client
        .create_partner(NewPartner {
            name: "サンプルパートナー".into(),
            ..Default::default()
        })
        .unwrap()
        .unwrap();
    println!("created partner: {:#?}", partner);
    // 取引先を更新
    let partner = client.update_partner(
        &partner.id,
        UpdatePartner {
            memo: Some("更新しました".into()),
            departments: vec![
                UpdateDepartmentInfo {
                    // IDがあると既存の部門を更新します
                    id: Some(partner.departments[0].clone().id),
                    name: Some("部門名です".into()),
                    ..Default::default()
                },
                UpdateDepartmentInfo {
                    // IDがなければ新規作成になります
                    name: Some("新しい部門".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    );
    println!("updated partner: {:#?}", partner);
    // 既存の取引先を列挙
    let partners = client.list_partners(1, 100).unwrap().unwrap();
    println!("{:#?}", partners.meta);
    for partner in partners.partners.iter() {
        println!("{:#?}", partner);
    }
    // 先程作成した取引先を削除
    println!("deleting {}", partner.id);
    client.delete_partner(&partner.id).unwrap().unwrap();


}
