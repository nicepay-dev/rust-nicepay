use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInfo {
    pub msId: String,
}

impl AdditionalInfo {
    pub fn new(msId: String) -> Self {
        Self { msId }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckBalanceModel {
    pub accountNo: String,
    pub additionalInfo: AdditionalInfo,
}

impl CheckBalanceModel {
    pub fn new() -> CheckBalanceModelBuilder {
        CheckBalanceModelBuilder::new()
    }
}

pub struct CheckBalanceModelBuilder {
    accountNo: Option<String>,
    additionalInfo: Option<AdditionalInfo>,
}

impl CheckBalanceModelBuilder {
    pub fn new() -> Self {
        Self {
            accountNo: None,
            additionalInfo: None,
        }
    }

    pub fn account_no(mut self, accountNo: String) -> Self {
        self.accountNo = Some(accountNo);
        self
    }

    pub fn additional_info(mut self, additionalInfo: AdditionalInfo) -> Self {
        self.additionalInfo = Some(additionalInfo);
        self
    }

    pub fn build(self) -> Result<CheckBalanceModel, String> {
        Ok(CheckBalanceModel {
            accountNo: self.accountNo.ok_or("accountNo is required")?,
            additionalInfo: self.additionalInfo.ok_or("additionalInfo is required")?,
        })
    }
}

impl Default for CheckBalanceModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}