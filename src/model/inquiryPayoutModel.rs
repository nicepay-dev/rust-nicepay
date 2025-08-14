use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InquiryPayoutModel {
    pub merchantId: String,
    pub originalReferenceNo: String,
    pub originalPartnerReferenceNo: String,
    pub beneficiaryAccountNo: String,
}

impl InquiryPayoutModel {
    pub fn new() -> InquiryPayoutModelBuilder {
        InquiryPayoutModelBuilder::new()
    }
}

pub struct InquiryPayoutModelBuilder {
    merchantId: Option<String>,
    originalReferenceNo: Option<String>,
    originalPartnerReferenceNo: Option<String>,
}

impl InquiryPayoutModelBuilder {
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

    pub fn beneficiary_account_no(mut self, beneficiaryAccountNo: String) -> Self {
        self.beneficiaryAccountNo = Some(beneficiaryAccountNo);
        self
    }

    pub fn build(self) -> Result<InquiryPayoutModel, String> {
        Ok(InquiryPayoutModel {
            merchantId: self.merchantId.ok_or("merchantId is required")?,
            originalReferenceNo: self.originalReferenceNo.ok_or("originalReferenceNo is required")?,
            originalPartnerReferenceNo: self.originalPartnerReferenceNo.ok_or("originalPartnerReferenceNo is required")?,
        })
    }
}

impl Default for InquiryPayoutModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}