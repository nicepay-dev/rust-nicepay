use crate::models::v2::approve_payout_model::ApprovePayoutV2Model;
use crate::models::v2::cancel_payout_model::CancelPayoutV2Model;
use crate::models::v2::check_balance_payout_model::CheckBalanceV2Model;
use crate::models::v2::inquiry_payout_model::InquiryPayoutV2Model;
use crate::models::v2::regist_payout_model::RegistPayoutV2Model;
use crate::models::v2::reject_payout_model::RejectPayoutV2Model;
use crate::utils::config::Config;
// use crate::utils::helper::Helper;
use reqwest::{Client, Error, Response};
use std::future::Future;
use std::pin::Pin;

pub type MyFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

pub struct PayoutV2Requester<'a> {
    pub config: &'a Config,
}

impl<'a> PayoutV2Requester<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn request_regist_payout(&self, req_payout: RegistPayoutV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/requestPayout", base_url);

        let client = Client::new();
        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_inquiry_payout(&self, req_payout: InquiryPayoutV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/inquiryPayout", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_approve_payout(&self, req_payout: ApprovePayoutV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/approvePayout", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_cancel_payout(&self, req_payout: CancelPayoutV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/cancelPayout", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_reject_payout(&self, req_payout: RejectPayoutV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/rejectPayout", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }

    pub fn request_check_balance(&self, req_payout: CheckBalanceV2Model) -> MyFuture<Response> {
        let base_url = self.config.get_snap_api_base_url();
        let url = format!("{}/api/direct/v2/balanceInquiry", base_url);

        let client = Client::new();

        let fut = async move {
            let response = client.post(url).json(&req_payout).send().await?;
            Ok(response)
        };

        Box::pin(fut)
    }
}
