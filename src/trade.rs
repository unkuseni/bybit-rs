use crate::api::{Trade, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    AmendOrderRequest, AmendOrderResponse, CancelOrderRequest, CancelOrderResponse,
    CancelallRequest, CancelallResponse, Category, OpenOrdersRequest, OpenOrdersResponse,
    OrderHistoryRequest, OrderHistoryResponse, OrderRequest, OrderResponse, OrderStatus, OrderType,
    Orders, RequestValue, Side, TradeHistoryRequest, TradeHistoryResponse, TradeHistorySummary,
};
use crate::util::{build_json_request, build_request, date_to_milliseconds, generate_random_uid};

use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Trader {
    pub client: Client,
    pub recv_window: u64,
}

/// Creates an order with various options for different account types and contract types.
///
/// # Account Coverage
/// - Unified account: Spot, USDT perpetual, USDC contract, Inverse contract, Option
/// - Classic account: Spot, USDT perpetual, Inverse contract
///
/// # Order Types
/// - Limit Order: Must specify quantity and price.
/// - Market Order: Executes at the best market price. Price parameter can be empty.
/// - Conditional Order: Set `triggerPrice` to convert to a conditional order.
///
/// # Time In Force Strategies
/// - GTC (Good Till Cancelled)
/// - IOC (Immediate Or Cancel)
/// - FOK (Fill Or Kill)
/// - PostOnly: Cancelled if it would be filled immediately when submitted.
///
/// # Take Profit / Stop Loss
/// - Can be set during order placement and modified later.
///
/// # Order Quantity
/// - Only positive numbers are supported for perpetual contract orders.
///
/// # Order Price
/// - Required for limit orders. Must be higher than liquidation price if in position.
///
/// # Order Link ID
/// - Custom active order ID up to 36 characters.
///
/// # Order Limits
/// - Futures: 500 active orders per contract, 10 conditional orders per account.
/// - Spot: 500 total orders, 30 open TP/SL orders, 30 open conditional orders.
/// - Option: 50 open orders.
///
/// # Rate Limit
/// - Refer to the rate limit table. Contact client manager for increases.
///
/// # Risk Control
/// - Bybit monitors API requests. Exceeding daily limits may lead to restrictions.
///
/// # Spot Stop Order in Different Account Types
/// - Classic account: New order ID upon stop order trigger.
/// - Unified account: Order ID remains unchanged upon stop order trigger.
impl Trader {
    pub async fn place_custom_order<'a>(&self, req: OrderRequest<'a>) -> Result<OrderStatus> {
        let mut parameters: BTreeMap<String, RequestValue> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            RequestValue::Str(req.category.as_str().into()),
        );
        parameters.insert("symbol".into(), RequestValue::Str(req.symbol.into()));
        if let Some(leverage) = req.is_leverage {
            if leverage {
                // Whether to borrow. Valid for Unified spot only. 0(default): false then spot trading, 1: true then margin trading
                parameters.insert("leverage".into(), RequestValue::Num(1.into()));
            }
        }
        parameters.insert("side".into(), RequestValue::Str(req.side.as_str().into()));
        parameters.insert(
            "orderType".into(),
            RequestValue::Str(req.order_type.as_str().into()),
        );
        // Market, Limit
        // Order quantity
        // UTA account
        // Spot: set marketUnit for market order qty unit, quoteCoin for market buy by default, baseCoin for market sell by default
        // Perps, Futures & Option: always use base coin as unit
        // Classic account
        // Spot: the unit of qty is quote coin for market buy order, for others, it is base coin
        // Perps, Futures: always use base coin as unit
        // Perps & Futures: if you pass qty="0" and reduceOnly="true", you can close the whole position of current symbol
        parameters.insert("qty".into(), RequestValue::Str(req.qty.to_string()));
        if let Some(market_unit) = req.market_unit {
            // The unit for qty when create Spot market orders for UTA account
            // baseCoin: for example, buy BTCUSDT, then "qty" unit is BTC
            // quoteCoin: for example, sell BTCUSDT, then "qty" unit is USDT/
            parameters.insert(
                "marketUnit".into(),
                RequestValue::Str(market_unit.to_string()),
            );
        }
        if let Some(price) = req.price {
            // Market order will ignore this field
            // If you have positions open, price needs  to be better than liquidation price
            parameters.insert("price".into(), RequestValue::Str(price.to_string()));
        }
        if let Some(trigger_direction) = req.trigger_direction {
            if trigger_direction {
                // triggered when market rises to trigger price
                parameters.insert("triggerDirection".into(), RequestValue::Num(1.into()));
            } else {
                // triggered when market falls to trigger price
                parameters.insert("triggerDirection".into(), RequestValue::Num(2.into()));
            }
        }
        if let Some(order_filter) = req.order_filter {
            // if itsn't passed, order b6y default
            parameters.insert("orderFilter".into(), RequestValue::Str(order_filter.into()));
        }
        if let Some(trigger_price) = req.trigger_price {
            // price << trigger price || price > trigger price
            parameters.insert(
                "triggerPrice".into(),
                RequestValue::Str(trigger_price.to_string()),
            );
        }
        if let Some(trigger) = req.trigger_by {
            parameters.insert("triggerBy".into(), RequestValue::Str(trigger.into()));
        }
        if let Some(iv) = req.order_iv {
            parameters.insert("orderIv".into(), RequestValue::Str(iv.to_string()));
        }
        if let Some(time_in_force) = req.time_in_force {
            parameters.insert(
                "timeInForce".into(),
                RequestValue::Str(time_in_force.into()),
            );
        }
        if let Some(position_idx) = req.position_idx {
            parameters.insert("positionIdx".into(), RequestValue::Num(position_idx.into()));
        }
        if let Some(order_link_id) = req.order_link_id {
            parameters.insert(
                "orderLinkId".into(),
                RequestValue::Str(order_link_id.into()),
            );
        } else {
            let uuid = generate_random_uid(36);
            parameters.insert("orderLinkId".into(), RequestValue::Str(uuid));
        }
        if let Some(price) = req.take_profit {
            parameters.insert("takeProfit".into(), RequestValue::Str(price.to_string()));
        }
        if let Some(price) = req.stop_loss {
            parameters.insert("stopLoss".into(), RequestValue::Str(price.to_string()));
        }
        if let Some(kind) = req.tp_trigger_by {
            parameters.insert("tpTriggerBy".into(), RequestValue::Str(kind.into()));
        }
        if let Some(kind) = req.sl_trigger_by {
            parameters.insert("slTriggerBy".into(), RequestValue::Str(kind.into()));
        }
        if let Some(reduce) = req.reduce_only {
            parameters.insert("reduceOnly".into(), RequestValue::Bool(reduce));
        }
        if let Some(close) = req.close_on_trigger {
            parameters.insert("closeOnTrigger".into(), RequestValue::Bool(close));
        }
        if let Some(v) = req.mmp {
            parameters.insert("mmp".into(), RequestValue::Bool(v));
        }
        if let Some(v) = req.tpsl_mode {
            parameters.insert("tpslMode".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.tp_limit_price {
            parameters.insert("tpTriggerPrice".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.sl_limit_price {
            parameters.insert("slTriggerPrice".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.tp_order_type {
            parameters.insert("tpOrderType".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.sl_order_type {
            parameters.insert("slOrderType".into(), RequestValue::Str(v.into()));
        }
        let request = build_json_request(&parameters);
        let response: OrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Place), 5000, Some(request))
            .await?;
        Ok(response.result)
    }

    pub async fn place_futures_limit_order(
        &self,
        category: Category,
        symbol: &str,
        side: Side,
        qty: f64,
        price: f64,
        mode: u8,
    ) -> Result<OrderResponse> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        let req = OrderRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            side,
            qty,
            order_type: OrderType::Limit,
            position_idx: Some(mode),
            order_link_id: Some(generate_random_uid(36).into()),
            price: Some(price),
            ..Default::default()
        };
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into_owned());
        parameters.insert("orderType".into(), req.order_type.as_str().into());
        parameters.insert("side".into(), req.side.as_str().into());
        if let Some(v) = req.order_link_id {
            parameters.insert("orderLinkId".into(), v.into());
        }
        parameters.insert("qty".into(), req.qty.to_string());
        if let Some(v) = req.position_idx {
            match v {
                0 | 1 | 2 => {
                    parameters.insert("positionIdx".into(), v.to_string());
                }
                _ => return Err("Invalid position index".into()),
            }
        }
        if let Some(v) = req.price {
            parameters.insert("price".into(), v.to_string());
        }
        parameters.insert("timeInForce".into(), "GTC".into());
        let request = build_json_request(&parameters);
        let response: OrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Place), 5000, Some(request))
            .await?;
        Ok(response)
    }

    pub async fn amend_order<'a>(&self, req: AmendOrderRequest<'a>) -> Result<OrderStatus> {
        let mut parameters: BTreeMap<String, RequestValue> = BTreeMap::new();
        parameters.insert(
            "category".into(),
            RequestValue::Str(req.category.as_str().into()),
        );
        parameters.insert("symbol".into(), RequestValue::Str(req.symbol.into()));
        if let Some(v) = req.order_id {
            parameters.insert("orderId".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.order_link_id {
            parameters.insert("orderLinkId".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.order_iv {
            parameters.insert("orderIv".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.trigger_price {
            parameters.insert("triggerPrice".into(), RequestValue::Str(v.to_string()));
        }
        parameters.insert("qty".into(), RequestValue::Num(req.qty.into()));
        if let Some(v) = req.price {
            parameters.insert("price".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.tpsl_mode {
            parameters.insert("tpslMode".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.take_profit {
            parameters.insert("takeProfit".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.stop_loss {
            parameters.insert("stopLoss".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.tp_trigger_by {
            parameters.insert("tpTriggerBy".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.sl_trigger_by {
            parameters.insert("slTriggerBy".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.trigger_by {
            parameters.insert("triggerBy".into(), RequestValue::Str(v.into()));
        }
        if let Some(v) = req.tp_limit_price {
            parameters.insert("tpLimitPrice".into(), RequestValue::Str(v.to_string()));
        }
        if let Some(v) = req.sl_limit_price {
            parameters.insert("slLimitPrice".into(), RequestValue::Str(v.to_string()));
        }
        let request = build_json_request(&parameters);
        let response: AmendOrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Amend), 5000, Some(request))
            .await?;
        Ok(response.result)
    }
    pub async fn cancel_order<'a>(&self, req: CancelOrderRequest<'a>) -> Result<OrderStatus> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(v) = req.order_id {
            parameters.insert("orderId".into(), v.into());
        }
        if let Some(v) = req.order_link_id {
            parameters.insert("orderLinkId".into(), v.into());
        }
        if let Some(v) = req.order_filter {
            parameters.insert("orderFilter".into(), v.into());
        }
        let request = build_json_request(&parameters);
        let response: CancelOrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Cancel), 5000, Some(request))
            .await?;
        Ok(response.result)
    }
    pub async fn get_open_orders<'a>(&self, req: OpenOrdersRequest<'a>) -> Result<Vec<Orders>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());

        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(settle_coin) = req.settle_coin {
            parameters.insert("settleCoin".into(), settle_coin.into());
        }
        if let Some(order_id) = req.order_id {
            parameters.insert("orderId".into(), order_id.into());
        }
        if let Some(order_link_id) = req.order_link_id {
            parameters.insert("orderLinkId".into(), order_link_id.into());
        }
        if let Some(open_only) = req.open_only {
            if matches!(open_only, 0 | 1 | 2) {
                parameters.insert("openOnly".into(), open_only.to_string().into());
            }
        }
        if let Some(order_filter) = req.order_filter {
            parameters.insert("orderFilter".into(), order_filter.into());
        }
        if let Some(limit) = req.limit {
            parameters.insert("limit".into(), limit.to_string().into());
        }

        let request = build_request(&parameters);
        let response: OpenOrdersResponse = self
            .client
            .get_signed(API::Trade(Trade::OpenOrders), 5000, Some(request))
            .await?;

        Ok(response.result.list)
    }
    pub async fn cancel_all_orders<'a>(
        &self,
        req: CancelallRequest<'a>,
    ) -> Result<Vec<OrderStatus>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        parameters.insert("symbol".into(), req.symbol.into());
        if let Some(base_coin) = req.base_coin {
            parameters.insert("baseCoin".into(), base_coin.into());
        }
        if let Some(settle_coin) = req.settle_coin {
            parameters.insert("settleCoin".into(), settle_coin.into());
        }
        if let Some(order_filter) = req.order_filter {
            parameters.insert("orderFilter".into(), order_filter.into());
        }
        if let Some(stop_order_type) = req.stop_order_type {
            parameters.insert("stopOrderType".into(), stop_order_type.into());
        }
        let request = build_json_request(&parameters);
        let response: CancelallResponse = self
            .client
            .post_signed(API::Trade(Trade::CancelAll), 5000, Some(request))
            .await?;
        Ok(response.result.list)
    }

    /// Retrieves the order history based on the given request parameters.
    ///
    /// # Arguments
    /// * `req` - An instance of `OrderHistoryRequest` containing the request parameters.
    ///
    /// # Returns
    /// A `Result` wrapping `OrderHistory` which contains the historical orders' data.
    /// If the operation fails, it returns an error.
    ///
    pub async fn get_order_history<'a>(&self, req: OrderHistoryRequest<'a>) -> Result<Vec<Orders>> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        req.symbol
            .map(|symbol| parameters.insert("symbol".into(), symbol.into()));
        req.base_coin
            .map(|base_coin| parameters.insert("baseCoin".into(), base_coin.into()));
        req.settle_coin
            .map(|settle_coin| parameters.insert("settleCoin".into(), settle_coin.into()));
        req.order_id
            .map(|order_id| parameters.insert("orderId".into(), order_id.into()));
        req.order_link_id
            .map(|order_link_id| parameters.insert("orderLinkId".into(), order_link_id.into()));
        req.order_filter
            .map(|order_filter| parameters.insert("orderFilter".into(), order_filter.into()));
        req.order_status
            .map(|order_status| parameters.insert("orderStatus".into(), order_status.into()));
        req.start_time
            .and_then(|start_time| Some(date_to_milliseconds(start_time.as_ref())))
            .map(|start_millis| parameters.insert("startTime".into(), start_millis.to_string()));
        req.end_time
            .and_then(|end_time| Some(date_to_milliseconds(end_time.as_ref())))
            .map(|end_millis| parameters.insert("endTime".into(), end_millis.to_string()));
        req.limit
            .map(|limit| parameters.insert("limit".into(), limit.to_string()));

        let request = build_request(&parameters);
        let response: OrderHistoryResponse = self
            .client
            .get_signed(API::Trade(Trade::History), 5000, Some(request))
            .await?;
        Ok(response.result.list)
    }
    pub async fn get_trade_history<'a>(
        &self,
        req: TradeHistoryRequest<'a>,
    ) -> Result<TradeHistorySummary> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("category".into(), req.category.as_str().into());
        req.symbol
            .map(|symbol| parameters.insert("symbol".into(), symbol.into()));
        req.order_id
            .map(|order_id| parameters.insert("orderId".into(), order_id.into()));
        req.order_link_id
            .map(|order_link_id| parameters.insert("orderLinkId".into(), order_link_id.into()));
        req.base_coin
            .map(|base_coin| parameters.insert("baseCoin".into(), base_coin.into()));
        req.start_time
            .and_then(|start_time| Some(date_to_milliseconds(start_time.as_ref())))
            .map(|start_millis| parameters.insert("startTime".into(), start_millis.to_string()));
        req.end_time
            .and_then(|end_time| Some(date_to_milliseconds(end_time.as_ref())))
            .map(|end_millis| parameters.insert("endTime".into(), end_millis.to_string()));
        req.limit
            .map(|limit| parameters.insert("limit".into(), limit.to_string()));
        req.exec_type
            .map(|exec_type| parameters.insert("execType".into(), exec_type.into()));
        let request = build_request(&parameters);
        let response: TradeHistoryResponse = self
            .client
            .get_signed(API::Trade(Trade::TradeHistory), 5000, Some(request))
            .await?;
        Ok(response.result)
    }
    pub async fn batch_place_order(&self) {}
    pub async fn batch_amend_order(&self) {}
    pub async fn batch_cancel_order(&self) {}
    pub async fn get_borrow_quota_spot(&self) {}
    pub async fn set_dcp_options(&self) {}
}
