use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InquiryVacctBuilder {
    partnerServiceId: Option<String>,
    customerNo: Option<String>,
    virtualAccountNo: Option<String>,
    inquiryRequestId: Option<String>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl InquiryVacctBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn partner_service_id(mut self, id: String) -> Self {
        self.partnerServiceId = Some(id);
        self
    }

    pub fn customer_no(mut self, no: String) -> Self {
        self.customerNo = Some(no);
        self
    }

    pub fn virtual_account_no(mut self, no: String) -> Self {
        self.virtualAccountNo = Some(no);
        self
    }

    pub fn inquiry_request_id(mut self, id: String) -> Self {
        self.inquiryRequestId = Some(id);
        self
    }

    pub fn additional_info(mut self, info: Vec<AdditionalInfo>) -> Self {
        self.additionalInfo = Some(info);
        self
    }

    pub fn build(self) -> Result<VacctRequest, &'static str> {
        Ok(VacctRequest {
            partnerServiceId: self.partnerServiceId.ok_or("partnerServiceId missing")?,
            customerNo: self.customerNo.ok_or("customerNo missing")?,
            virtualAccountNo: self.virtualAccountNo.ok_or("virtualAccountNo missing")?,
            inquiryRequestId: self.inquiryRequestId.ok_or("inquiryRequestId missing")?,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo missing")?,
        })
    }
}

pub struct AdditionalInfo {
    pub trxId: String,
    pub tXidVA: String,
    pub totalAmount: TotalAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,
    pub currency: String,
}
