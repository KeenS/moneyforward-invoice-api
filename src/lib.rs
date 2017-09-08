extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[macro_use]
extern crate log;

pub mod model;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use reqwest::{Client as HttpClient, Url, Method};
pub use model::*;

pub type ApiResult<T> = ::std::result::Result<T, ApiError>;
pub type Result<T> = reqwest::Result<ApiResult<T>>;


static SERVER: &str = "https://invoice.moneyforward.com/";

pub struct Client {
    client: HttpClient,
    server: Url,
    token: String,
}

impl Client {
    pub fn new<S: Into<String>>(token: S) -> ::std::result::Result<Self, Box<::std::error::Error>> {
        Ok(Self {
            token: token.into(),
            client: HttpClient::new().map_err(Box::new)?,
            server: SERVER.parse().map_err(Box::new)?,
        })
    }
}

impl Client {
    pub fn get_office(&mut self) -> Result<Office> {
        self.get("/api/v1/office.json")
    }

    pub fn update_office(&mut self, req: UpdateOffice) -> Result<Office> {
        self.patch_json("/api/v1/office", &req)
    }

    pub fn list_partners(&mut self, page: u32, per_page: u32) -> Result<Partners> {
        // TBD: check if per_page <= 100 ?
        self.get_params(
            "/api/v1/partners.json",
            &[
                ("page", &page.to_string()),
                ("per_page", &per_page.to_string()),
            ],
        )
    }

    pub fn get_partner(&mut self, id: &str) -> Result<Partner> {
        self.get(&format!("/api/v1/partners/{}.json", id))
    }

    pub fn create_partner(&mut self, req: NewPartner) -> Result<Partner> {
        #[derive(Serialize)]
        struct Request {
            partner: NewPartner,
        }

        self.post_json("/api/v1/partners", &Request { partner: req })
    }

    pub fn update_partner(&mut self, id: &str, req: UpdatePartner) -> Result<Partner> {
        #[derive(Serialize)]
        struct Request {
            partner: UpdatePartner,
        }
        self.patch_json(
            &format!("/api/v1/partners/{}", id),
            &Request { partner: req },
        )
    }

    pub fn delete_partner(&mut self, id: &str) -> Result<()> {
        self.delete_void(&format!("/api/v1/partners/{}.json", id))
    }

    pub fn list_billings(&mut self, page: u32, per_page: u32) -> Result<Billings> {
        // TBD: check if per_page <= 100 ?
        self.get_params(
            "/api/v1/billings.json",
            &[
                ("page", &page.to_string()),
                ("per_page", &per_page.to_string()),
            ],
        )
    }

    pub fn search_billings(
        &mut self,
        page: u32,
        per_page: u32,
        q: &str,
        range_key: Option<&str>,
        from: Option<NaiveDate>,
        to: Option<NaiveDate>,
    ) -> Result<BillingQueryResponse> {
        let page = page.to_string();
        let per_page = per_page.to_string();
        let from = from.map(|date| date.to_string());
        let to = to.map(|date| date.to_string());
        let mut params: Vec<(&str, &str)> =
            vec![("page", &page), ("per_page", &per_page), ("q", q)];
        if let Some(range_key) = range_key {
            params.push(("range_key", range_key));
        }
        if let Some(ref from) = from {
            params.push(("from", from));
        }
        if let Some(ref to) = to {
            params.push(("to", to));
        }
        self.get_params("/api/v1/billings/search.json", &params)
    }

    pub fn get_billing(&mut self, id: &str) -> Result<Billing> {
        self.get(&format!("/api/v1/billing/{}.json", id))
    }

    pub fn get_billing_pdf(&mut self, id: &str) -> Result<BillingPdf> {
        self.request_raw::<()>(
            Method::Get,
            &format!("/api/v1/billing/{}.pdf", id),
            None,
            None,
        ).map(|res| res.map(BillingPdf))
    }

    pub fn create_billing(&mut self, req: NewBilling) -> Result<Billing> {
        #[derive(Serialize)]
        struct Request {
            billing: NewBilling,
        }
        self.post_json("/api/v1/billings", &Request { billing: req })
    }

    pub fn update_billing(&mut self, id: &str, req: UpdateBilling) -> Result<Billing> {
        #[derive(Serialize)]
        struct Request {
            billing: UpdateBilling,
        }
        self.patch_json(
            &format!("/api/v1/billings/{}", id),
            &Request { billing: req },
        )
    }

    pub fn posting_billing(&mut self, id: &str) -> Result<()> {
        self.post_void(&format!("/api/v1/billings/{}/posting", id))
    }

    pub fn cancel_posting_billing(&mut self, id: &str) -> Result<()> {
        self.post_void(&format!("/api/v1/billings/{}/cancel_posting", id))
    }

    pub fn delete_billing(&mut self, id: &str) -> Result<()> {
        self.delete_void(&format!("/api/v1/billings/{}", id))
    }

    pub fn list_items(&mut self) -> Result<Items> {
        self.get("/api/v1/items.json")
    }

    pub fn get_item(&mut self, id: &str) -> Result<Item> {
        self.get(&format!("/api/v1/items/{}.json", id))
    }
    pub fn create_item(&mut self, req: NewItem) -> Result<Item> {
        self.post_json("/api/v1/items.json", &req)
    }

    pub fn update_item(&mut self, id: &str, req: UpdateItem) -> Result<Item> {
        self.patch_json(&format!("/api/v1/items/{}", id), &req)
    }

    pub fn delete_item(&mut self, id: &str) -> Result<()> {
        self.delete_void(&format!("/api/v1/items/{}", id))
    }

    pub fn sent_history(&mut self, page: u32, per_page: u32) -> Result<SentHistories> {
        self.get_params(
            "/api/v1/sent_history.json",
            &[
                ("page", &page.to_string()),
                ("per_page", &per_page.to_string()),
            ],
        )
    }
}


impl Client {
    fn request_raw<Req>(
        &self,
        method: Method,
        path: &str,
        params: Option<&[(&str, &str)]>,
        data: Option<&Req>,
    ) -> Result<reqwest::Response>
    where
        Req: Serialize,
    {
        use reqwest::header::{Authorization, Bearer};

        let mut url = self.server.join(path).unwrap();
        if let Some(params) = params {
            url.query_pairs_mut().extend_pairs(params);
        }

        let mut reqbuilder = self.client.request(method, url)?;
        reqbuilder.header(Authorization(Bearer { token: self.token.clone() }));
        if let Some(data) = data {
            reqbuilder.json(data)?;
        }

        debug!("request: {:?}", reqbuilder);
        let mut res = reqbuilder.send()?;
        debug!("response: {:?}", res);
        if res.status().is_success() {
            Ok(Ok(res))
        } else {
            Ok(Err(res.json()?))
        }
    }


    fn request<Req, Res>(
        &self,
        method: Method,
        path: &str,
        params: Option<&[(&str, &str)]>,
        data: Option<&Req>,
    ) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        match self.request_raw(method, path, params, data) {
            Ok(Ok(mut res)) => {
                use std::io::Read;
                let mut s = String::new();
                res.read_to_string(&mut s).unwrap();
                debug!("response json: {}", s);
                Ok(Ok(serde_json::from_str(&s).unwrap()))
            }
            Ok(Err(e)) => Ok(Err(e)),
            Err(e) => Err(e),
        }
    }

    fn get<Res>(&mut self, path: &str) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.request::<(), _>(Method::Get, path, None, None)
    }

    fn get_params<Res>(&mut self, path: &str, params: &[(&str, &str)]) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.request::<(), _>(Method::Get, path, Some(params), None)
    }

    fn post_json<Req, Res>(&mut self, path: &str, req: &Req) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.request(Method::Post, path, None, Some(req))
    }

    fn post_void(&mut self, path: &str) -> Result<()> {
        self.request_raw::<()>(Method::Post, path, None, None).map(
            |res| res.map(|_| ()),
        )
    }

    fn patch_json<Req, Res>(&mut self, path: &str, req: &Req) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.request(Method::Patch, path, None, Some(req))
    }

    fn delete_void(&mut self, path: &str) -> Result<()> {
        self.request_raw::<()>(Method::Delete, path, None, None)
            .map(|res| res.map(|_| ()))
    }
}
