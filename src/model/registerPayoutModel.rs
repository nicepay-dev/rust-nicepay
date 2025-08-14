use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

impl Amount {
    pub fn new(value: String, currency: String) -> Self {
        Self { value, currency }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayoutModel {
    pub merchantId: String,
    pub msId: String,
    pub beneficiaryAccountNo: String,
    pub beneficiaryName: String,
    pub beneficiaryPhone: String,
    pub beneficiaryCustomerResidence: String,
    pub beneficiaryCustomerType: String,
    pub beneficiaryPostalCode: String,
    pub payoutMethod: String,
    pub beneficiaryBankCode: String,
    pub amount: Amount,
    pub partnerReferenceNo: String,
    pub description: String,
    pub reservedDt: String,
    pub reservedTm: String,
}

impl RegisterPayoutModel {
    pub fn new() -> RegisterPayoutModelBuilder {
        RegisterPayoutModelBuilder::new()
    }
}

pub struct RegisterPayoutModelBuilder {
    merchantId: Option<String>,
    msId: Option<String>,
    beneficiaryAccountNo: Option<String>,
    beneficiaryName: Option<String>,
    beneficiaryPhone: Option<String>,
    beneficiaryCustomerResidence: Option<String>,
    beneficiaryCustomerType: Option<String>,
    beneficiaryPostalCode: Option<String>,
    payoutMethod: Option<String>,
    beneficiaryBankCode: Option<String>,
    amount: Option<Amount>,
    partnerReferenceNo: Option<String>,
    description: Option<String>,
    reservedDt: Option<String>,
    reservedTm: Option<String>,
}

impl RegisterPayoutModelBuilder {
    pub fn new() -> Self {
        Self {
            merchantId: None,
            msId: None,
            beneficiaryAccountNo: None,
            beneficiaryName: None,
            beneficiaryPhone: None,
            beneficiaryCustomerResidence: None,
            beneficiaryCustomerType: None,
            beneficiaryPostalCode: None,
            payoutMethod: None,
            beneficiaryBankCode: None,
            amount: None,
            partnerReferenceNo: None,
            description: None,
            reservedDt: None,
            reservedTm: None,
        }
    }

    pub fn merchant_id(mut self, merchantId: String) -> Self {
        self.merchantId = Some(merchantId);
        self
    }

    pub fn ms_id(mut self, msId: String) -> Self {
        self.msId = Some(msId);
        self
    }

    pub fn beneficiary_account_no(mut self, beneficiaryAccountNo: String) -> Self {
        self.beneficiaryAccountNo = Some(beneficiaryAccountNo);
        self
    }

    pub fn beneficiary_name(mut self, beneficiaryName: String) -> Self {
        self.beneficiaryName = Some(beneficiaryName);
        self
    }

    pub fn beneficiary_phone(mut self, beneficiaryPhone: String) -> Self {
        self.beneficiaryPhone = Some(beneficiaryPhone);
        self
    }

    pub fn beneficiary_customer_residence(mut self, beneficiaryCustomerResidence: String) -> Self {
        self.beneficiaryCustomerResidence = Some(beneficiaryCustomerResidence);
        self
    }

    pub fn beneficiary_customer_type(mut self, beneficiaryCustomerType: String) -> Self {
        self.beneficiaryCustomerType = Some(beneficiaryCustomerType);
        self
    }

    pub fn beneficiary_postal_code(mut self, beneficiaryPostalCode: String) -> Self {
        self.beneficiaryPostalCode = Some(beneficiaryPostalCode);
        self
    }

    pub fn payout_method(mut self, payoutMethod: String) -> Self {
        self.payoutMethod = Some(payoutMethod);
        self
    }

    pub fn beneficiary_bank_code(mut self, beneficiaryBankCode: String) -> Self {
        self.beneficiaryBankCode = Some(beneficiaryBankCode);
        self
    }

    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn partner_reference_no(mut self, partnerReferenceNo: String) -> Self {
        self.partnerReferenceNo = Some(partnerReferenceNo);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn reserved_dt(mut self, reservedDt: String) -> Self {
        self.reservedDt = Some(reservedDt);
        self
    }

    pub fn reserved_tm(mut self, reservedTm: String) -> Self {
        self.reservedTm = Some(reservedTm);
        self
    }

    pub fn build(self) -> Result<RegisterPayoutModel, String> {
        Ok(RegisterPayoutModel {
            merchantId: self.merchantId.ok_or("merchantId is required")?,
            beneficiaryAccountNo: self.beneficiaryAccountNo.ok_or("beneficiaryAccountNo is required")?,
            beneficiaryName: self.beneficiaryName.ok_or("beneficiaryName is required")?,
            beneficiaryPhone: self.beneficiaryPhone.ok_or("beneficiaryPhone is required")?,
            beneficiaryCustomerResidence: self.beneficiaryCustomerResidence.ok_or("beneficiaryCustomerResidence is required")?,
            beneficiaryCustomerType: self.beneficiaryCustomerType.ok_or("beneficiaryCustomerType is required")?,
            payoutMethod: self.payoutMethod.ok_or("payoutMethod is required")?,
            beneficiaryBankCode: self.beneficiaryBankCode.ok_or("beneficiaryBankCode is required")?,
            amount: self.amount.ok_or("amount is required")?,
            partnerReferenceNo: self.partnerReferenceNo.ok_or("partnerReferenceNo is required")?,
            description: self.description.ok_or("description is required")?,
        })
    }
}

impl Default for RegisterPayoutModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}