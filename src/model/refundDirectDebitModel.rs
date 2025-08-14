use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundDirectDebitRequest {
    pub merchantId: String,
    pub subMerchantId: Option<String>,
    pub originalPartnerReferenceNo: String,
    pub originalReferenceNo: String,
    pub partnerRefundNo: String,
    pub refundAmount: RefundAmount,
    pub externalStoreId: Option<String>,
    pub reason: String,
    pub additionalInfo: Option<AdditionalInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundAmount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdditionalInfo {
    pub refundType: String,
}

#[derive(Debug, Clone)]
pub struct RefundDirectDebitBuilder {
    merchantId: Option<String>,
    subMerchantId: Option<String>,
    originalPartnerReferenceNo: Option<String>,
    originalReferenceNo: Option<String>,
    partnerRefundNo: Option<String>,
    refundAmount: Option<RefundAmount>,
    externalStoreId: Option<String>,
    reason: Option<String>,
    additionalInfo: Option<AdditionalInfo>,
}

impl RefundDirectDebitBuilder {
    pub fn new() -> Self {
        Self {
            merchantId: None,
            subMerchantId: None,
            originalPartnerReferenceNo: None,
            originalReferenceNo: None,
            partnerRefundNo: None,
            refundAmount: None,
            externalStoreId: None,
            reason: None,
            additionalInfo: None,
        }
    }

    pub fn merchant_id(mut self, merchant_id: String) -> Self {
        self.merchantId = Some(merchant_id);
        self
    }

    pub fn sub_merchant_id(mut self, sub_merchant_id: String) -> Self {
        self.subMerchantId = Some(sub_merchant_id);
        self
    }

    pub fn original_partner_reference_no(mut self, original_partner_reference_no: String) -> Self {
        self.originalPartnerReferenceNo = Some(original_partner_reference_no);
        self
    }

    pub fn original_reference_no(mut self, original_reference_no: String) -> Self {
        self.originalReferenceNo = Some(original_reference_no);
        self
    }

    pub fn partner_refund_no(mut self, partner_refund_no: String) -> Self {
        self.partnerRefundNo = Some(partner_refund_no);
        self
    }

    pub fn refund_amount(mut self, value: String, currency: String) -> Self {
        self.refundAmount = Some(RefundAmount { value, currency });
        self
    }

    pub fn external_store_id(mut self, external_store_id: String) -> Self {
        self.externalStoreId = Some(external_store_id);
        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    pub fn additional_info(mut self, refund_type: String) -> Self {
        self.additionalInfo = Some(AdditionalInfo {
            refundType: refund_type,
        });
        self
    }

    pub fn build(self) -> Result<RefundDirectDebitRequest, &'static str> {
        Ok(RefundDirectDebitRequest {
            merchantId: self.merchantId.ok_or("merchantId is required")?,
            subMerchantId: self.subMerchantId,
            originalPartnerReferenceNo: self
                .originalPartnerReferenceNo
                .ok_or("originalPartnerReferenceNo is required")?,
            originalReferenceNo: self
                .originalReferenceNo
                .ok_or("originalReferenceNo is required")?,
            partnerRefundNo: self.partnerRefundNo.ok_or("partnerRefundNo is required")?,
            refundAmount: self.refundAmount.ok_or("refundAmount is required")?,
            externalStoreId: self.externalStoreId,
            reason: self.reason.ok_or("reason is required")?,
            additionalInfo: self.additionalInfo,
        })
    }
}

impl Default for RefundDirectDebitBuilder {
    fn default() -> Self {
        Self::new()
    }
}
