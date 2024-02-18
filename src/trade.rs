use serde_json::{json, Value};

use crate::api::{Trade, API};
use crate::client::Client;
use crate::errors::Result;
use crate::model::{
    AmendOrderRequest, AmendOrderResponse, AmendedOrder, BatchAmendRequest, BatchAmendResponse,
    BatchCancelRequest, BatchCancelResponse, BatchPlaceRequest, BatchPlaceResponse, BatchedOrder,
    CancelOrderRequest, CancelOrderResponse, CancelallRequest, CancelallResponse, CanceledOrder,
    Category, OpenOrdersRequest, OpenOrdersResponse, OrderConfirmation, OrderHistoryRequest,
    OrderHistoryResponse, OrderRequest, OrderResponse, OrderStatus, OrderType, Orders, Side,
    TradeHistoryRequest, TradeHistoryResponse, TradeHistorySummary,
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
pub enum Action<'a> {
    Order(OrderRequest<'a>, bool),
    Amend(AmendOrderRequest<'a>),
    Cancel(CancelOrderRequest<'a>),
}
impl Trader {
    pub async fn place_custom_order<'a>(&self, req: OrderRequest<'a>) -> Result<OrderStatus> {
        let action = Action::Order(req, false);
        let parameters = Self::build_orders(action);

        let request = build_json_request(&parameters);
        let response: OrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Place), self.recv_window.into(), Some(request))
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
            .post_signed(API::Trade(Trade::Place), self.recv_window.into(), Some(request))
            .await?;
        Ok(response)
    }

    pub async fn amend_order<'a>(&self, req: AmendOrderRequest<'a>) -> Result<OrderStatus> {
        let action = Action::Amend(req);
        let parameters = Self::build_orders(action);
        let request = build_json_request(&parameters);
        let response: AmendOrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Amend), self.recv_window.into(), Some(request))
            .await?;
        Ok(response.result)
    }
    pub async fn cancel_order<'a>(&self, req: CancelOrderRequest<'a>) -> Result<OrderStatus> {
        let action = Action::Cancel(req);
        let parameters = Self::build_orders(action);
        let request = build_json_request(&parameters);
        let response: CancelOrderResponse = self
            .client
            .post_signed(API::Trade(Trade::Cancel), self.recv_window.into(), Some(request))
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
            .post_signed(API::Trade(Trade::CancelAll), self.recv_window.into(), Some(request))
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
            .get_signed(API::Trade(Trade::History), self.recv_window.into(), Some(request))
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
            .get_signed(API::Trade(Trade::TradeHistory), self.recv_window.into(), Some(request))
            .await?;
        Ok(response.result)
    }
    pub async fn batch_place_order<'a>(
        &self,
        req: BatchPlaceRequest<'a>,
    ) -> Result<Vec<(BatchedOrder, OrderConfirmation)>> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            _ => {
                println!("Invalid category");
            }
        }
        let mut requests_array: Vec<Value> = Vec::new();
        for value in req.requests {
            let action = Action::Order(value, true);
            let order_object = Self::build_orders(action); // Assuming this returns the correct object structure
            let built_orders = json!(order_object);
            requests_array.push(built_orders);
        }
        parameters.insert("request".into(), Value::Array(requests_array));
        let request = build_json_request(&parameters);
        println!("{:?}", request);
        let response: BatchPlaceResponse = self
            .client
            .post_signed(API::Trade(Trade::BatchPlace), self.recv_window.into(), Some(request))
            .await?;
        if response.result.list.len() != response.ret_ext_info.list.len() {
            println!("List length not equal");
        }
        let paired_list: Vec<(BatchedOrder, OrderConfirmation)> = response
            .result
            .list
            .into_iter()
            .zip(response.ret_ext_info.list.into_iter())
            .collect();
        Ok(paired_list)
    }
    pub async fn batch_amend_order<'a>(
        &self,
        req: BatchAmendRequest<'a>,
    ) -> Result<Vec<(AmendedOrder, OrderConfirmation)>> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            _ => {
                println!("Invalid category");
            }
        }
        let mut requests_array: Vec<Value> = Vec::new();
        for value in req.requests {
            let action = Action::Amend(value);
            let amend_object = Self::build_orders(action); // Assuming this returns the correct object structure
            let built_amends = json!(amend_object);
            requests_array.push(built_amends);
        }
        parameters.insert("request".into(), Value::Array(requests_array));
        let request = build_json_request(&parameters);
        println!("{:?}", request);
        let response: BatchAmendResponse = self
            .client
            .post_signed(API::Trade(Trade::BatchAmend), self.recv_window.into(), Some(request))
            .await?;
        if response.result.list.len() != response.ret_ext_info.list.len() {
            println!("List length not equal");
        }
        let paired_list: Vec<(AmendedOrder, OrderConfirmation)> = response
            .result
            .list
            .into_iter()
            .zip(response.ret_ext_info.list.into_iter())
            .collect();
        Ok(paired_list)
    }

    pub async fn batch_cancel_order<'a>(
        &self,
        req: BatchCancelRequest<'a>,
    ) -> Result<Vec<(CanceledOrder, OrderConfirmation)>> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match req.category {
            Category::Linear | Category::Inverse | Category::Option => {
                parameters.insert("category".into(), req.category.as_str().into());
            }
            _ => {
                println!("Invalid category");
            }
        }
        let mut requests_array: Vec<Value> = Vec::new();
        for value in req.requests {
            let action = Action::Cancel(value);
            let cancel_object = Self::build_orders(action); // Assuming this returns the correct object structure
            let built_cancels = json!(cancel_object);
            requests_array.push(built_cancels);
        }
        parameters.insert("request".into(), Value::Array(requests_array));
        let request = build_json_request(&parameters);
        let response: BatchCancelResponse = self
            .client
            .post_signed(API::Trade(Trade::BatchCancel), self.recv_window.into(), Some(request))
            .await?;
        if response.result.list.len() != response.ret_ext_info.list.len() {
            println!("List length not equal");
        }
        let paired_list: Vec<(CanceledOrder, OrderConfirmation)> = response
            .result
            .list
            .into_iter()
            .zip(response.ret_ext_info.list.into_iter())
            .collect();
        Ok(paired_list)
    }
    pub async fn get_borrow_quota_spot(&self) {
        // TODO: Implement this function
        todo!("This function has not yet been implemented");
    }
    pub async fn set_dcp_options(&self) {
        // TODO: Implement this function
        todo!("This function has not yet been implemented");
    }

    pub fn build_orders<'a>(action: Action<'a>) -> BTreeMap<String, Value> {
        let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
        match action {
            Action::Order(req, batch_order) => {
                if batch_order == false {
                    parameters.insert("category".into(), req.category.as_str().into());
                }
                parameters.insert("symbol".into(), req.symbol.into());
                if let Some(leverage) = req.is_leverage {
                    if leverage {
                        // Whether to borrow. Valid for Unified spot only. 0(default): false then spot trading, 1: true then margin trading
                        parameters.insert("leverage".into(), 1.into());
                    }
                }
                parameters.insert("side".into(), req.side.as_str().into());
                parameters.insert("orderType".into(), req.order_type.as_str().into());

                parameters.insert("qty".into(), req.qty.to_string().into());
                if let Some(market_unit) = req.market_unit {
                    parameters.insert("marketUnit".into(), market_unit.to_string().into());
                }
                if let Some(price) = req.price {
                    parameters.insert("price".into(), price.to_string().into());
                }
                if let Some(trigger_direction) = req.trigger_direction {
                    if trigger_direction {
                        parameters.insert("triggerDirection".into(), 1.into());
                    } else {
                        parameters.insert("triggerDirection".into(), 2.into());
                    }
                }
                if let Some(order_filter) = req.order_filter {
                    parameters.insert("orderFilter".into(), order_filter.into());
                }
                if let Some(trigger_price) = req.trigger_price {
                    parameters.insert("triggerPrice".into(), trigger_price.to_string().into());
                }
                if let Some(trigger) = req.trigger_by {
                    parameters.insert("triggerBy".into(), trigger.into());
                }
                if let Some(iv) = req.order_iv {
                    parameters.insert("orderIv".into(), iv.to_string().into());
                }
                if let Some(time_in_force) = req.time_in_force {
                    parameters.insert("timeInForce".into(), time_in_force.into());
                }
                if let Some(v) = req.position_idx {
                    match v {
                        0 | 1 | 2 => {
                            parameters.insert("positionIdx".into(), v.to_string().into());
                        }
                        _ => println!("Invalid position idx"),
                    }
                }
                if let Some(order_link_id) = req.order_link_id {
                    parameters.insert("orderLinkId".into(), order_link_id.into());
                } else {
                    let uuid = generate_random_uid(36);
                    parameters.insert("orderLinkId".into(), uuid.into());
                }
                if let Some(price) = req.take_profit {
                    parameters.insert("takeProfit".into(), price.to_string().into());
                }
                if let Some(price) = req.stop_loss {
                    parameters.insert("stopLoss".into(), price.to_string().into());
                }
                if let Some(kind) = req.tp_trigger_by {
                    parameters.insert("tpTriggerBy".into(), kind.into());
                }
                if let Some(kind) = req.sl_trigger_by {
                    parameters.insert("slTriggerBy".into(), kind.into());
                }
                if let Some(reduce) = req.reduce_only {
                    parameters.insert("reduceOnly".into(), reduce.into());
                }
                if let Some(close) = req.close_on_trigger {
                    parameters.insert("closeOnTrigger".into(), close.into());
                }
                if let Some(v) = req.mmp {
                    parameters.insert("mmp".into(), v.into());
                }
                if let Some(v) = req.tpsl_mode {
                    parameters.insert("tpslMode".into(), v.into());
                }
                if let Some(v) = req.tp_limit_price {
                    parameters.insert("tpTriggerPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.sl_limit_price {
                    parameters.insert("slTriggerPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.tp_order_type {
                    parameters.insert("tpOrderType".into(), v.into());
                }
                if let Some(v) = req.sl_order_type {
                    parameters.insert("slOrderType".into(), v.into());
                }
            }
            Action::Amend(req) => {
                parameters.insert("category".into(), req.category.as_str().into());
                parameters.insert("symbol".into(), req.symbol.into());
                if let Some(v) = req.order_id {
                    parameters.insert("orderId".into(), v.into());
                }
                if let Some(v) = req.order_link_id {
                    parameters.insert("orderLinkId".into(), v.into());
                }
                if let Some(v) = req.order_iv {
                    parameters.insert("orderIv".into(), v.to_string().into());
                }
                if let Some(v) = req.trigger_price {
                    parameters.insert("triggerPrice".into(), v.to_string().into());
                }
                parameters.insert("qty".into(), req.qty.into());
                if let Some(v) = req.price {
                    parameters.insert("price".into(), v.to_string().into());
                }
                if let Some(v) = req.tpsl_mode {
                    parameters.insert("tpslMode".into(), v.into());
                }
                if let Some(v) = req.take_profit {
                    parameters.insert("takeProfit".into(), v.to_string().into());
                }
                if let Some(v) = req.stop_loss {
                    parameters.insert("stopLoss".into(), v.to_string().into());
                }
                if let Some(v) = req.tp_trigger_by {
                    parameters.insert("tpTriggerBy".into(), v.into());
                }
                if let Some(v) = req.sl_trigger_by {
                    parameters.insert("slTriggerBy".into(), v.into());
                }
                if let Some(v) = req.trigger_by {
                    parameters.insert("triggerBy".into(), v.into());
                }
                if let Some(v) = req.tp_limit_price {
                    parameters.insert("tpLimitPrice".into(), v.to_string().into());
                }
                if let Some(v) = req.sl_limit_price {
                    parameters.insert("slLimitPrice".into(), v.to_string().into());
                }
            }
            Action::Cancel(req) => {
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
            }
        }
        parameters
    }
}
