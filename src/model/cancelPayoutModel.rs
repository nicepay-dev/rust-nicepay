use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPayoutModel {
    pub merchantId: String,
    pub originalReferenceNo: String,
    pub originalPartnerReferenceNo: String,
}

impl CancelPayoutModel {
    pub fn new() -> CancelPayoutModelBuilder {
        CancelPayoutModelBuilder::new()
    }
}

pub struct CancelPayoutModelBuilder {
    merchantId: Option<String>,
    originalReferenceNo: Option<String>,
    originalPartnerReferenceNo: Option<String>,
}

impl CancelPayoutModelBuilder {
    pub fn new() -> Self {
        Self {
            merchantId: None,
            originalReferenceNo: None,
            originalPartnerReferenceNo: None,
        }
    }

    pub fn merchant_id(mut self, merchantId: String) -> Self {
        self.merchantId = Some(merchantId);
        self
    }

    pub fn original_reference_no(mut self, originalReferenceNo: String) -> Self {
        self.originalReferenceNo = Some(originalReferenceNo);
        self
    }

    pub fn original_partner_reference_no(mut self, originalPartnerReferenceNo: String) -> Self {
        self.originalPartnerReferenceNo = Some(originalPartnerReferenceNo);
        self
    }

    pub fn build(self) -> Result<CancelPayoutModel, String> {
        Ok(CancelPayoutModel {
            merchantId: self.merchantId.ok_or("merchantId is required")?,
            originalReferenceNo: self.originalReferenceNo.ok_or("originalReferenceNo is required")?,
            originalPartnerReferenceNo: self.originalPartnerReferenceNo.ok_or("originalPartnerReferenceNo is required")?,
        })
    }
}

impl Default for CancelPayoutModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}