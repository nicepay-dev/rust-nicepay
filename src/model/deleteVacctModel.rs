use serde::{Debug, Default, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteVacctBuilder {
    partnerServiceId: Option<String>,
    customerNo: Option<String>,
    virtualAccountNo: Option<String>,
    virtualAccountName: Option<String>,
    trxId: Option<String>,
    totalAmount: Option<TotalAmount>,
    additionalInfo: Option<Vec<AdditionalInfo>>,
}

impl DeleteVacctBuilder {
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

    pub fn trx_id(mut self, id: String) -> Self {
        self.trxId = Some(id);
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
            trxId: self.trxId.ok_or("trxId missing")?,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo missing")?,
        })
    }
}

pub struct AdditionalInfo {
    pub tXidVA: String,
    pub cancelMessage: String,
    pub totalAmount: TotalAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: String,
    pub currency: String,
}
