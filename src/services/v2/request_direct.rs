use crate::models::v2::cancel_direct_model::CancelDirectV2Model;
use crate::models::v2::inquiry_direct_model::InquiryDirectV2Model;
use crate::models::v2::payment_direct_model::PaymentDirectV2Model;
use crate::models::v2::regist_direct_model::RegistDirectV2Model;
use crate::utils::config::Config;
// use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct DirectV2Requester<'a> {
    pub config: &'a Config,
}

impl<'a> DirectV2Requester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist(&self, req_direct: RegistDirectV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/direct/v2/registration", base_url);

        let client = Client::new();
        let fut = async move {
            let response = client.post(url).json(&req_direct).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_inquiry(&self, req_direct: InquiryDirectV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/direct/v2/inquiry", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_direct).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_payment(&self, req_direct: PaymentDirectV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/direct/v2/payment", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_direct).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_cancel(&self, req_direct: CancelDirectV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/direct/v2/cancel", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_direct).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
