use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryQRBuilder {
    originalReferenceNo: Option<String>,
    serviceCode: Option<String>,
    merchantId: Option<String>,
    originalPartnerReferenceNo: Option<String>,
    externalStoreId: Option<String>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl QueryQRBuilder {
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

    pub fn additional_info(mut self, info: Vec<AdditionalInfo>) -> Self {
        self.additionalInfo = Some(info);
        self
    }

    pub fn build(self) -> Result<QrisRequest, &'static str> {
        Ok(QrisRequest {
            originalReferenceNo: self
                .originalReferenceNo
                .ok_or("originalReferenceNo missing")?,
            merchantId: self.merchantId.ok_or("merchantId missing")?,
            external_store_id: self.storeId.ok_or("external_store_id missing")?,
            service_code: self.service_code.ok_or("service_code missing")?,
            original_partner_reference_no: self
                .original_partner_reference_no
                .ok_or("original_partner_reference_no missing")?,
        })
    }
}

pub struct AdditionalInfo {}
