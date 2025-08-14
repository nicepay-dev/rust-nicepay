use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenerateQRBuilder {
    partnerReferenceNo: Option<String>,
    amount: Option<Amount>,
    merchantId: Option<String>,
    storeId: Option<String>,
    validityPeriod: Option<String>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl GenerateQRBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn partner_reference_no(mut self, id: String) -> Self {
        self.partnerReferenceNo = Some(id);
        self
    }

    pub fn merchant_id(mut self, no: String) -> Self {
        self.merchantId = Some(no);
        self
    }

    pub fn store_id(mut self, no: String) -> Self {
        self.storeId = Some(no);
        self
    }

    pub fn validity_period(mut self, name: String) -> Self {
        self.validityPeriod = Some(name);
        self
    }

    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn additional_info(mut self, info: Vec<AdditionalInfo>) -> Self {
        self.additionalInfo = Some(info);
        self
    }

    pub fn build(self) -> Result<QrisRequest, &'static str> {
        Ok(QrisRequest {
            partnerReferenceNo: self
                .partnerReferenceNo
                .ok_or("partnerReferenceNo missing")?,
            merchantId: self.merchantId.ok_or("merchantId missing")?,
            storeId: self.storeId.ok_or("storeId missing")?,
            validityPeriod: self.validityPeriod.ok_or("validityPeriod missing")?,
            amount: self.amount.ok_or("amount missing")?,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo missing")?,
        })
    }
}

pub struct AdditionalInfo {
    pub goodsNm: String,
    pub dbProcessUrl: String,
    pub billingNm: String,
    pub billingPhone: String,
    pub billingEmail: String,
    pub billingCity: String,
    pub billingAddr: String,
    pub billingState: String,
    pub billingPostCd: String,
    pub billingCountry: String,
    pub userIP: String,
    pub cartData: String,
    pub mitraCd: String,
    pub msId: String,
    pub msFee: String,
    pub msFeeType: String,
    pub mbFee: String,
    pub mbFeeType: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}
