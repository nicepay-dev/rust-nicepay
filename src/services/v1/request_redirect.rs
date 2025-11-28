use crate::models::v1::regist_redirect_model::RegistRedirectV1Model;
use crate::utils::config::Config;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

#[derive(Debug)]  // ✅ Add this
pub struct RedirectV1Requester<'a> {
    pub config: &'a Config,
}

impl<'a> RedirectV1Requester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist(&self, req_redirect: RegistRedirectV1Model) -> MyFuture<Response> {
        
        let base_url = self.config.get_v1_base_url();
        let url = format!("{}/orderRegist.do", base_url);


        let client = Client::new();

        let fut = async move {
            let response = client
                .post(url)
                .form(&req_redirect)
                .send()
                .await?;

            Ok(response)
        };

        Box::pin(fut)
    }
}