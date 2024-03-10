use std::collections::BTreeMap;

use crate::api::{Account, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    AccountInfo, AccountInfoResponse, BatchSetCollateralCoinResponse, BorrowHistoryRequest, BorrowHistoryResponse, Category, CollateralInfoList, CollateralInfoResponse, FeeRate, FeeRateResponse, LiabilityQty, MarginModeResult, RepayLiabilityResponse, SetCollateralCoinResponse, SetMarginModeResponse, SmpResponse, SmpResult, SpotHedgingResponse, SwitchList, TransactionLogRequest, TransactionLogResponse, TransactionLogResult, UTAResponse, UTAUpdateStatus, WalletList, WalletResponse
};

use serde_json::{json, Value};

use crate::util::{build_json_request, build_request, date_to_milliseconds};

#[derive(Clone)]
pub struct AccountManager {
    pub client: Client,
    pub recv_window: u64,
}

impl AccountManager {
    pub async fn get_wallet_balance(
        &self,
        account: &str,
        coin: Option<&str>,
    ) -> Result<WalletList> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("accountType".into(), account.into());
        if let Some(c) = coin {
            parameters.insert("coin".into(), c.into());
        }
        let request = build_request(&parameters);
        let response: WalletResponse = self
            .client
            .get_signed(
                API::Account(Account::Balance),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn upgrade_to_uta(&self) -> Result<UTAUpdateStatus> {
        let response: UTAResponse = self
            .client
            .post_signed(
                API::Account(Account::UpgradetoUTA),
                self.recv_window.into(),
                None,
            )
            .await?;
        Ok(response.result)
    }

    pub async fn get_borrow_history<'a>(
        &self,
        req: BorrowHistoryRequest<'_>,
    ) -> Result<BorrowHistoryResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(c) = req.coin {
            parameters.insert("coin".into(), c.into());
        }
        if let Some(end_str) = req.start_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| end_millis.into());
        }
        if let Some(end_str) = req.end_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_millis.into());
        }
        if let Some(s) = req.limit {
            parameters.insert("limit".into(), s.into());
        }
        let request = build_request(&parameters);
        let response: BorrowHistoryResponse = self
            .client
            .get_signed(
                API::Account(Account::BorrowHistory),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn repay_liability(&self, coin: Option<&str>) -> Result<LiabilityQty> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(c) = coin {
            parameters.insert("coin".into(), c.into());
        }
        let request = build_json_request(&parameters);
        let response: RepayLiabilityResponse = self
            .client
            .post_signed(
                API::Account(Account::RepayLiability),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn set_collateral_coin(
        &self,
        coin: &str,
        switch: bool,
    ) -> Result<SetCollateralCoinResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("coin".into(), coin.into());
        if switch == true {
            parameters.insert("collateralSwitch".into(), "ON".into());
        } else {
            parameters.insert("collateralSwitch".into(), "OFF".into());
        }
        let request = build_json_request(&parameters);
        let response: SetCollateralCoinResponse = self
            .client
            .post_signed(
                API::Account(Account::SetCollateral),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn batch_set_collateral(&self, requests: Vec<(&str, bool)>) -> Result<SwitchList> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        let mut requests_array: Vec<Value> = Vec::new();
        for (coin, switch) in requests {
            let mut build_switch: BTreeMap<String, Value> = BTreeMap::new();
            build_switch.insert("coin".into(), coin.into());
            build_switch.insert("collateralSwitch".into(), switch.into());
            let build_switches = json!(&build_switch);
            requests_array.push(build_switches);
        }
        parameters.insert("request".into(), Value::Array(requests_array));
        let request = build_json_request(&parameters);
        let response: BatchSetCollateralCoinResponse = self
            .client
            .post_signed(
                API::Account(Account::BatchSetCollateral),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn get_collateral_info(&self, coin: Option<&str>) -> Result<CollateralInfoList> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(v) = coin {
            parameters.insert("currency".into(), v.into());
        }
        let req = build_request(&parameters);
        let response: CollateralInfoResponse = self
            .client
            .get_signed(
                API::Account(Account::CollateralInfo),
                self.recv_window.into(),
                Some(req),
            )
            .await?;
        Ok(response.result)
    }
    pub async fn get_fee_rate(&self, category: Category, symbol: Option<String>) -> Result<Vec<FeeRate>> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), category.as_str().into());
        if let Some(s) = symbol {
            parameters.insert("symbol".into(), s.into());
        }
        let req = build_request(&parameters);
        let response: FeeRateResponse= self
            .client
            .post_signed(
                API::Account(Account::FeeRate),
                self.recv_window.into(),
                Some(req),
            )
            .await?;
        Ok(response.result.list)
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        let response: AccountInfoResponse = self
            .client
            .get_signed(
                API::Account(Account::Information),
                self.recv_window.into(),
                None,
            )
            .await?;
        Ok(response.result)
    }

    pub async fn get_transaction_log<'a>(
        &self,
        req: TransactionLogRequest<'a>,
    ) -> Result<TransactionLogResult> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(v) = req.account_type {
            parameters.insert("accountType".into(), v.into());
        }
        if let Some(v) = req.category {
            parameters.insert("category".into(), v.as_str().into());
        }

        if let Some(s) = req.currency {
            parameters.insert("currency".into(), s.into());
        }
        if let Some(c) = req.base_coin {
            parameters.insert("baseCoin".into(), c.into());
        }
        if let Some(t) = req.log_type {
            parameters.insert("type".into(), t.into());
        }
        if let Some(start_str) = req.start_time.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("startTime".to_owned())
                .or_insert_with(|| start_millis.into());
        }
        if let Some(end_str) = req.end_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("endTime".to_owned())
                .or_insert_with(|| end_millis.into());
        }
        if let Some(s) = req.limit {
            parameters.insert("limit".into(), s.into());
        }

        let request = build_request(&parameters);
        let response: TransactionLogResponse = self
            .client
            .get_signed(
                API::Account(Account::TransactionLog),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn get_smp_id(&self) -> Result<SmpResult> {
        let response: SmpResponse = self
            .client
            .get_signed(
                API::Account(Account::SMPGroupID),
                self.recv_window.into(),
                None,
            )
            .await?;
        Ok(response.result)
    }

    pub async fn set_margin_mode(&self, margin_mode: &str) -> Result<MarginModeResult> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("setMarginMode".into(), margin_mode.into());
        let request = build_json_request(&parameters);
        let response: SetMarginModeResponse = self
            .client
            .post_signed(
                API::Account(Account::SetMarginMode),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response.result)
    }

    pub async fn set_spot_hedging(&self, spot_hedging: bool) -> Result<SpotHedgingResponse> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if spot_hedging == true {
            parameters.insert("setHedgingMode".into(), "ON".into());
        } else {
            parameters.insert("setHedgingMode".into(), "OFF".into());
        }
        let request = build_json_request(&parameters);
        let response: SpotHedgingResponse = self
            .client
            .post_signed(
                API::Account(Account::SetSpotHedging),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }
}
