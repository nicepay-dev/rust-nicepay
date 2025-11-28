use crate::models::v1::regist_direct_model::RegistDirectV1Model;
use crate::models::v1::inquiry_direct_model::InquiryDirectV1Model;
use crate::models::v1::cancel_direct_model::CancelDirectV1Model;
use crate::models::v1::one_pass_token::OnePassTokenJsonData;
use crate::models::v1::secure_ve_request::SecureVeRequest;
use crate::models::v1::migs_request::{self, MigsRequest};
use crate::utils::config::Config;

use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

#[derive(Debug)]
pub struct DirectV1Requester<'a> {
    pub config: &'a Config,
    client: Client,
}

impl<'a> DirectV1Requester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.config.get_v1_base_url(), path)
    }

    fn post_form<T>(&self, url: String, body: T) -> MyFuture<Response>
    where
        T: serde::Serialize + Send + 'static,
    {
        let client = self.client.clone();

        let fut = async move {
            let rsp = client.post(url).form(&body).send().await?;
            Ok(rsp)
        };

        Box::pin(fut)
    }

    // -------------------------
    // NICEPAY DIRECT APIs
    // -------------------------

    pub fn request_regist(&self, req: RegistDirectV1Model) -> MyFuture<Response> {
        let url = self.endpoint("onePass.do");
        self.post_form(url, req)
    }

    pub fn request_inquiry(&self, req: InquiryDirectV1Model) -> MyFuture<Response> {
        let url = self.endpoint("onePassStatus.do");
        self.post_form(url, req)
    }

    pub fn request_cancel(&self, req: CancelDirectV1Model) -> MyFuture<Response> {
        let url = self.endpoint("onePassAllCancel.do");
        self.post_form(url, req)
    }

    // -------------------------
    // onePassToken.do
    // -------------------------
    pub fn request_one_pass_token(&self, json_data: OnePassTokenJsonData) -> MyFuture<Response> {
        let base_url = self.config.get_v1_base_url();
        let url = format!("{}/onePassToken.do", base_url);

        let client = self.client.clone();

        let json_string = serde_json::to_string(&json_data)
            .expect("Failed to encode JSON for jsonData");  

        let fut = async move {
            let response = client
                .post(url)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&[("jsonData", json_string)])
                .send()
                .await?;

            Ok(response)
        };

        Box::pin(fut)
    }


    // -------------------------
    // secureVeRequest.do
    // -------------------------

    pub fn request_secure_ve(&self, req: SecureVeRequest) -> MyFuture<Response> {
        let base_url = self.config.get_v1_base_url();
        let url = format!("{}/secureVeRequest.do", base_url);


        let client = self.client.clone();

        Box::pin(async move {
            client
                .post(url)
                .query(&req)   
                .send()
                .await
        })
    }


    // -------------------------
    // migsRequest.do
    // -------------------------

    pub fn migs_request(&self, req: MigsRequest) -> MyFuture<Response> {
        let base_url = self.config.get_v1_base_url();
        let url = format!("{}/migsRequest.do", base_url);


        let client = self.client.clone();

        Box::pin(async move {
            client
                .post(url)
                .query(&req)   
                .send()
                .await
        })
    }


}
