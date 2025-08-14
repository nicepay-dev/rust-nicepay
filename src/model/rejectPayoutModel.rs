use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectPayoutModel {
    pub merchantId: String,
    pub originalReferenceNo: String,
    pub originalPartnerReferenceNo: String,
}

impl RejectPayoutModel {
    pub fn new() -> RejectPayoutModelBuilder {
        RejectPayoutModelBuilder::new()
    }
}

pub struct RejectPayoutModelBuilder {
    merchantId: Option<String>,
    originalReferenceNo: Option<String>,
    originalPartnerReferenceNo: Option<String>,
}

impl RejectPayoutModelBuilder {
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

    pub fn build(self) -> Result<RejectPayoutModel, String> {
        Ok(RejectPayoutModel {
            merchantId: self.merchantId.ok_or("merchantId is required")?,
            originalReferenceNo: self.originalReferenceNo.ok_or("originalReferenceNo is required")?,
            originalPartnerReferenceNo: self.originalPartnerReferenceNo.ok_or("originalPartnerReferenceNo is required")?,
        })
    }
}

impl Default for RejectPayoutModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}