use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDirectDebitRequest {
    pub partnerReferenceNo: String,
    pub merchant_id: String,
    pub subMerchantId: Option<String>,
    pub amount: Amount,
    pub urlParam: Vec<UrlParam>,
    pub externalStoreId: Option<String>,
    pub validUpTo: Option<String>,
    pub pointOfInitiation: Option<String>,
    pub additionalInfo: AdditionalInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlParam {
    pub url: String,
    pub r#type: String,
    pub isDeeplink: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdditionalInfo {
    pub mitraCd: String,
    pub goodsNm: String,
    pub billingNm: String,
    pub billingPhone: String,
    pub billingEmail: String,
    pub callBackUrl: String,
    pub dbProcessUrl: String,
    pub cartData: String,
    pub msId: Option<String>,
    pub msFee: Option<String>,
    pub msFeeType: Option<String>,
    pub mbFee: Option<String>,
    pub mbFeeType: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateDirectDebitBuilder {
    partnerReferenceNo: Option<String>,
    merchant_id: Option<String>,
    subMerchantId: Option<String>,
    amount: Option<Amount>,
    urlParam: Vec<UrlParam>,
    externalStoreId: Option<String>,
    validUpTo: Option<String>,
    pointOfInitiation: Option<String>,
    additionalInfo: Option<AdditionalInfo>,
}

impl CreateDirectDebitBuilder {
    pub fn new() -> Self {
        Self {
            partnerReferenceNo: None,
            merchant_id: None,
            subMerchantId: None,
            amount: None,
            urlParam: Vec::new(),
            externalStoreId: None,
            validUpTo: None,
            pointOfInitiation: None,
            additionalInfo: None,
        }
    }

    pub fn partnerReferenceNo(mut self, partnerReferenceNo: String) -> Self {
        self.partnerReferenceNo = Some(partnerReferenceNo);
        self
    }

    pub fn merchant_id(mut self, merchant_id: String) -> Self {
        self.merchant_id = Some(merchant_id);
        self
    }

    pub fn subMerchantId(mut self, subMerchantId: String) -> Self {
        self.subMerchantId = Some(subMerchantId);
        self
    }

    pub fn amount(mut self, value: String, currency: String) -> Self {
        self.amount = Some(Amount { value, currency });
        self
    }

    pub fn add_urlParam(mut self, url: String, r#type: String, isDeeplink: Option<String>) -> Self {
        self.urlParam.push(UrlParam {
            url,
            r#type,
            isDeeplink,
        });
        self
    }

    pub fn externalStoreId(mut self, externalStoreId: String) -> Self {
        self.externalStoreId = Some(externalStoreId);
        self
    }

    pub fn validUpTo(mut self, validUpTo: String) -> Self {
        self.validUpTo = Some(validUpTo);
        self
    }

    pub fn pointOfInitiation(mut self, pointOfInitiation: String) -> Self {
        self.pointOfInitiation = Some(pointOfInitiation);
        self
    }

    pub fn additionalInfo(
        mut self,
        mitraCd: String,
        goodsNm: String,
        billingNm: String,
        billingPhone: String,
        billingEmail: String,
        callBackUrl: String,
        dbProcessUrl: String,
        cartData: String,
    ) -> Self {
        self.additionalInfo = Some(AdditionalInfo {
            mitraCd,
            goodsNm,
            billingNm,
            billingPhone,
            billingEmail,
            callBackUrl,
            dbProcessUrl,
            cartData,
            msId: None,
            msFee: None,
            msFeeType: None,
            mbFee: None,
            mbFeeType: None,
        });
        self
    }

    pub fn msId(mut self, msId: String) -> Self {
        if let Some(ref mut info) = self.additionalInfo {
            info.msId = Some(msId);
        }
        self
    }

    pub fn msFee(mut self, msFee: String) -> Self {
        if let Some(ref mut info) = self.additionalInfo {
            info.msFee = Some(msFee);
        }
        self
    }

    pub fn msFeeType(mut self, msFeeType: String) -> Self {
        if let Some(ref mut info) = self.additionalInfo {
            info.msFeeType = Some(msFeeType);
        }
        self
    }

    pub fn mbFee(mut self, mbFee: String) -> Self {
        if let Some(ref mut info) = self.additionalInfo {
            info.mbFee = Some(mbFee);
        }
        self
    }

    pub fn mbFeeType(mut self, mbFeeType: String) -> Self {
        if let Some(ref mut info) = self.additionalInfo {
            info.mbFeeType = Some(mbFeeType);
        }
        self
    }

    pub fn build(self) -> Result<CreateDirectDebitRequest, &'static str> {
        Ok(CreateDirectDebitRequest {
            partnerReferenceNo: self
                .partnerReferenceNo
                .ok_or("partnerReferenceNo is required")?,
            merchant_id: self.merchant_id.ok_or("merchant_id is required")?,
            amount: self.amount.ok_or("amount is required")?,
            urlParam: self.urlParam,
            externalStoreId: self.externalStoreId,
            validUpTo: self.validUpTo,
            pointOfInitiation: self.pointOfInitiation,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo is required")?,
        })
    }
}

impl Default for CreateDirectDebitBuilder {
    fn default() -> Self {
        Self::new()
    }
}
