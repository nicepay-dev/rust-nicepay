use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InquiryDirectDebitBuilder {
    merchantId: Option<String>,
    subMerchantId: Option<String>,
    originalReferenceNo: Option<String>,
    originalPartnerReferenceNo: Option<String>,
    serviceCode: Option<String>,
    transactionDate: Option<String>,
    externalStoreId: Option<String>,
    amount: Option<Amount>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl InquiryDirectDebitBuilder {
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

    pub fn service_code(mut self, name: String) -> Self {
        self.serviceCode = Some(name);
        self
    }

    pub fn original_partner_reference_no(mut self, no: String) -> Self {
        self.originalPartnerReferenceNo = Some(no);
        self
    }

    pub fn transaction_date(mut self, date: String) -> Self {
        self.transactionDate = Some(date);
        self
    }

    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn sub_merchant_id(mut self, id: String) -> Self {
        self.subMerchantId = Some(id);
        self
    }

    pub fn additional_info(mut self, info: Vec<AdditionalInfo>) -> Self {
        self.additionalInfo = Some(info);
        self
    }

    pub fn build(self) -> Result<DirectDebitRequest, &'static str> {
        Ok(DirectDebitRequest {
            originalReferenceNo: self
                .originalReferenceNo
                .ok_or("originalReferenceNo missing")?,
            merchantId: self.merchantId.ok_or("merchantId missing")?,
            service_code: self.service_code.ok_or("service_code missing")?,
            original_partner_reference_no: self
                .original_partner_reference_no
                .ok_or("original_partner_reference_no missing")?,
            amount: self.amount.ok_or("amount missing")?,
        })
    }
}

pub struct Amount {
    pub value: String,
    pub currency: String,
}

pub struct AdditionalInfo {}
