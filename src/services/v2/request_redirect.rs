use crate::models::v2::regist_redirect_model::RegistRedirectV2Model;
use crate::utils::config::Config;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct RedirectV2Requester<'a> {
    pub config: &'a Config,
}

impl<'a> RedirectV2Requester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist(&self, req_redirect: RegistRedirectV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/redirect/v2/registration", base_url);

        let client = Client::new();
        let fut = async move {
            let response = client.post(url).json(&req_redirect).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
