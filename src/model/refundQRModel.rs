use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RefundQRBuilder {
    originalReferenceNo: Option<String>,
    reason: Option<String>,
    merchantId: Option<String>,
    originalPartnerReferenceNo: Option<String>,
    partnerRefundNo: Option<String>,
    externalStoreId: Option<String>,
    refundAmount: Option<RefundAmount>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl RefundQRBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn original_reference_no(mut self, id: String) -> Self {
        self.originalReferenceNo = Some(id);
        self
    }

    pub fn merchant_id(mut self, no: String) -> Self {
        self.merchantId = Some(no);
        self
    }

    pub fn external_store_id(mut self, no: String) -> Self {
        self.externalStoreId = Some(no);
        self
    }

    pub fn reason(mut self, name: String) -> Self {
        self.reason = Some(name);
        self
    }

    pub fn partner_refund_no(mut self, id: String) -> Self {
        self.partnerRefundNo = Some(id);
        self
    }

    pub fn original_partner_reference_no(mut self, no: String) -> Self {
        self.originalPartnerReferenceNo = Some(no);
        self
    }

    pub fn additional_info(mut self, info: Vec<AdditionalInfo>) -> Self {
        self.additionalInfo = Some(info);
        self
    }

    pub fn refund_amount(mut self, amount: RefundAmount) -> Self {
        self.refundAmount = Some(amount);
        self
    }

    pub fn build(self) -> Result<QrisRequest, &'static str> {
        Ok(QrisRequest {
            originalReferenceNo: self
                .originalReferenceNo
                .ok_or("originalReferenceNo missing")?,
            merchantId: self.merchantId.ok_or("merchantId missing")?,
            external_store_id: self.storeId.ok_or("externalStoreId missing")?,
            original_partner_reference_no: self
                .original_partner_reference_no
                .ok_or("originalPartnerReferenceNo missing")?,
            reason: self.reason,
            partnerRefundNo: self.partnerRefundNo.ok_or("partnerRefundNo missing")?,
            refundAmount: self.refundAmount.ok_or("refundAmount missing")?,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo missing")?,
        })
    }
}

pub struct AdditionalInfo {
    cancelType: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundAmount {
    pub value: String,
    pub currency: String,
}
