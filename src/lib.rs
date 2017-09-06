extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod model;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use reqwest::{Client, Url, Method};
use model::*;

pub type ApiResult<T> = ::std::result::Result<T, ApiError>;
pub type Result<T> = reqwest::Result<ApiResult<T>>;


static SERVER: &str = "https://invoice.moneyforward.com/";

pub struct Api {
    client: Client,
    server: Url,
    token: String,
}

impl Api {
    pub fn new(token: String) -> ::std::result::Result<Self, Box<::std::error::Error>> {
        Ok(Self {
            token,
            client: Client::new().map_err(Box::new)?,
            server: SERVER.parse().map_err(Box::new)?,
        })
    }
}

impl Api {
    pub fn get_office(&mut self) -> Result<Office> {
        self.get("/api/v1/office.json")
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
        self.post_json("/api/v1/partners", &req)
    }

    pub fn update_partner(&mut self, req: UpdatePartner) -> Result<Partner> {
        self.patch_json("/api/v1/partners", &req)
    }

    pub fn delete_partner(&mut self, id: &str) -> Result<()> {
        // TODO: check `()` is treated as no content
        self.delete(&format!("/api/v1/partners/{}.json", id))
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
        self.post_json("/api/v1/billings", &req)
    }

    pub fn update_billing(&mut self, req: UpdateBilling) -> Result<Billing> {
        self.patch_json("/api/v1/billings", &req)
    }

    pub fn posting_billing(&mut self, id: &str) -> Result<()> {
        self.post(&format!("/api/v1/billings/{}/posting", id))
    }

    pub fn cancel_posting_billing(&mut self, id: &str) -> Result<()> {
        self.post(&format!("/api/v1/billings/{}/cancel_posting", id))
    }

    pub fn delete_billing(&mut self, id: &str) -> Result<()> {
        self.delete(&format!("/api/v1/billings/{}", id))
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

    pub fn update_item(&mut self, req: UpdateItem) -> Result<Item> {
        self.patch_json("/api/v1/items.json", &req)
    }

    pub fn delete_item(&mut self, id: &str) -> Result<()> {
        self.delete(&format!("/api/v1/items/{}", id))
    }

    pub fn sent_history(mut self) -> Result<SentHistory> {
        self.get("/api/v1/sent_history.json")
    }
}


impl Api {
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

        let mut res = reqbuilder.send()?;
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
            Ok(Ok(mut req)) => Ok(Ok(req.json()?)),
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

    fn post<Res>(&mut self, path: &str) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.request::<(), _>(Method::Post, path, None, None)
    }
    fn post_json<Req, Res>(&mut self, path: &str, req: &Req) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.request(Method::Post, path, None, Some(req))
    }

    fn patch_json<Req, Res>(&mut self, path: &str, req: &Req) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        self.request(Method::Patch, path, None, Some(req))
    }

    fn delete<Res>(&mut self, path: &str) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        self.request::<(), _>(Method::Delete, path, None, None)
    }
}
