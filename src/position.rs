use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::api::{Position, API};
use crate::client::Client;
use crate::errors::BybitError;
use crate::model::{
    AddMarginRequest, AddMarginResponse, AddReduceMarginRequest, AddReduceMarginResponse,
    ChangeMarginRequest, ChangeMarginResponse, ClosedPnlRequest,
    ClosedPnlResponse, InfoResponse, LeverageRequest, LeverageResponse,
    MarginModeRequest, MarginModeResponse, MoveHistoryRequest, MoveHistoryResponse,
    MovePositionRequest, MovePositionResponse, PositionRequest, SetRiskLimit, SetRiskLimitResponse, TradingStopRequest,
    TradingStopResponse,
};
use crate::util::{build_json_request, build_request, date_to_milliseconds};

#[derive(Clone)]
pub struct PositionManager<'a> {
    pub client: Client<'a>,
    pub recv_window: u16,
}

impl<'a> PositionManager<'_> {
    /// Asynchronously retrieves information about a position based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `req` - The position request containing the category, symbol, base coin, settle coin, and limit.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `PositionInfo` if the operation is successful, or an error if it fails.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::model::{PositionRequest, Category};
    /// use crate::errors::Result;
    /// use crate::api::PositionInfo;
    /// use my_module::PositionManager;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let position_manager = PositionManager::new();
    ///     let request = PositionRequest::new(Category::Linear, Some("symbol"), Some("base_coin"), Some("settle_coin"), Some(10));
    ///     let position_info = position_manager.get_info(request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_info<'b>(&self, req: PositionRequest<'_>) -> Result<InfoResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.base_coin {
            parameters.insert("baseCoin".into(), v.into());
        }
        if let Some(v) = req.settle_coin {
            parameters.insert("settleCoin".into(), v.into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.to_string());
        }
        let request = build_request(&parameters);
        let response: InfoResponse = self
            .client
            .get_signed(
                API::Position(Position::Information),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    // Sets the leverage for a given symbol.
    ///
    /// # Arguments
    ///
    /// * `req` - The leverage request containing category, symbol, and leverage.
    ///
    /// # Returns
    ///
    /// A result containing the leverage response.
    pub async fn set_leverage<'b>(
        &self,
        req: LeverageRequest<'_>,
    ) -> Result<LeverageResponse, BybitError> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("buyLeverage".into(), req.leverage.to_string());
        parameters.insert("sellLeverage".into(), req.leverage.to_string());
        let request = build_json_request(&parameters);
        let response: LeverageResponse = self
            .client
            .post_signed(
                API::Position(Position::SetLeverage),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the margin mode.
    ///
    /// # Arguments
    ///
    /// * `req` - The ChangeMarginRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<ChangeMarginResponse> - The result of setting the margin mode.
    pub async fn set_margin_mode<'b>(
        &self,
        req: ChangeMarginRequest<'_>,
    ) -> Result<ChangeMarginResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("tradeMode".into(), req.trade_mode.into());
        let request = build_json_request(&parameters);
        let response: ChangeMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchIsolated),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the position mode.
    ///
    /// # Arguments
    ///
    /// * `req` - The MarginModeRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<MarginModeResponse> - The result of setting the position mode.
    pub async fn set_position_mode<'b>(
        &self,
        req: MarginModeRequest<'_>,
    ) -> Result<MarginModeResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(v) = req.coin {
            parameters.insert("coin".into(), v.into());
        }
        parameters.insert("mode".into(), req.mode.into());
        let request = build_json_request(&parameters);
        let response: MarginModeResponse = self
            .client
            .post_signed(
                API::Position(Position::SwitchMode),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the risk limit.
    ///
    /// # Arguments
    ///
    /// * `req` - The SetRiskLimitRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<SetRiskLimitResult> - The result of setting the risk limit.
    pub async fn set_risk_limit<'b>(
        &self,
        req: SetRiskLimit<'_>,
    ) -> Result<SetRiskLimitResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("riskId".into(), req.risk_id.into());
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: SetRiskLimitResponse = self
            .client
            .post_signed(
                API::Position(Position::SetRiskLimit),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    /// Set the trading stop.
    ///
    /// # Arguments
    ///
    /// * `req` - The TradingStopRequest containing the necessary information.
    ///
    /// # Returns
    ///
    /// * Result<TradingStopResponse> - The result of setting the trading stop.
    pub async fn set_trading_stop<'b>(
        &self,
        req: TradingStopRequest<'_>,
    ) -> Result<TradingStopResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(v) = req.take_profit {
            parameters.insert("takeProfit".into(), v.into());
        }
        if let Some(v) = req.stop_loss {
            parameters.insert("stopLoss".into(), v.into());
        }
        if let Some(v) = req.tp_trigger_by {
            parameters.insert("tpTriggerBy".into(), v.into());
        }
        if let Some(v) = req.sl_trigger_by {
            parameters.insert("slTriggerBy".into(), v.into());
        }
        if let Some(v) = req.tpsl_mode {
            parameters.insert("tpslMode".into(), v.into());
        }
        if let Some(v) = req.tp_order_type {
            parameters.insert("tpOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.sl_order_type {
            parameters.insert("slOrderType".into(), v.as_str().into());
        }
        if let Some(v) = req.tp_size {
            parameters.insert("tpSize".into(), v.into());
        }
        if let Some(v) = req.sl_size {
            parameters.insert("slSize".into(), v.into());
        }
        if let Some(v) = req.tp_limit_price {
            parameters.insert("tpLimitPrice".into(), v.into());
        }
        if let Some(v) = req.sl_limit_price {
            parameters.insert("slLimitPrice".into(), v.into());
        }
        parameters.insert("positionIdx".into(), req.position_idx.into());
        let request = build_json_request(&parameters);
        let response: TradingStopResponse = self
            .client
            .post_signed(
                API::Position(Position::SetTradingStop),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn set_add_margin<'b>(
        &self,
        req: AddMarginRequest<'_>,
    ) -> Result<AddMarginResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if req.auto_add {
            parameters.insert("autoAddMargin".into(), 1.into());
        } else {
            parameters.insert("autoAddMargin".into(), 0.into());
        }
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: AddMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::SetAutoaddMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn add_or_reduce_margin<'b>(
        &self,
        req: AddReduceMarginRequest<'_>,
    ) -> Result<AddReduceMarginResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        parameters.insert("margin".into(), req.margin.into());
        if let Some(v) = req.position_idx {
            parameters.insert("positionIdx".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: AddReduceMarginResponse = self
            .client
            .post_signed(
                API::Position(Position::AddorReduceMargin),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn get_closed_pnl<'b>(
        &self,
        req: ClosedPnlRequest<'_>,
    ) -> Result<ClosedPnlResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }

        if let Some(start_str) = req.start_time.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| start_millis.to_string().into());
        }
        if let Some(end_str) = req.end_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string().into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.into());
        }
        let request = build_request(&parameters);
        let response: ClosedPnlResponse = self
            .client
            .get_signed(
                API::Position(Position::ClosedPnl),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn move_position<'b>(
        &self,
        req: MovePositionRequest<'_>,
    ) -> Result<MovePositionResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        parameters.insert("fromUid".into(), req.from_uid.into());
        parameters.insert("toUid".into(), req.to_uid.into());
        parameters.insert("list".into(), json!(req.list));
        let request = build_json_request(&parameters);
        let response: MovePositionResponse = self
            .client
            .post_signed(
                API::Position(Position::MovePosition),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }

    pub async fn move_position_history<'b>(
        &self,
        req: MoveHistoryRequest<'_>,
    ) -> Result<MoveHistoryResponse, BybitError> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        if let Some(v) = req.category {
            parameters.insert("category".into(), v.as_str().into());
        }
        if let Some(v) = req.symbol {
            parameters.insert("symbol".into(), v.into());
        }
        if let Some(start_str) = req.start_time.as_ref().map(|s| s.as_ref()) {
            let start_millis = date_to_milliseconds(start_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| start_millis.to_string().into());
        }
        if let Some(end_str) = req.end_time.as_ref().map(|s| s.as_ref()) {
            let end_millis = date_to_milliseconds(end_str);
            parameters
                .entry("end".to_owned())
                .or_insert_with(|| end_millis.to_string().into());
        }
        if let Some(v) = req.status {
            parameters.insert("status".into(), v.into());
        }
        if let Some(v) = req.block_trade_id {
            parameters.insert("blockTradeId".into(), v.into());
        }
        if let Some(v) = req.limit {
            parameters.insert("limit".into(), v.into());
        }
        let request = build_request(&parameters);
        let response: MoveHistoryResponse = self
            .client
            .get_signed(
                API::Position(Position::MovePositionHistory),
                self.recv_window.into(),
                Some(request),
            )
            .await?;
        Ok(response)
    }
}
