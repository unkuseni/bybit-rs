use crate::prelude::*;

/// Parameters for batch placing multiple orders.
///
/// Used to construct a request to the `/v5/order/create-batch` endpoint to place multiple orders simultaneously. This is useful for bots executing complex strategies, such as placing multiple limit orders at different price levels in perpetual futures.
#[derive(Clone, Default)]
pub struct BatchPlaceRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type for the batch orders.
    pub category: Category,

    /// A list of order requests to place.
    ///
    /// Contains the individual order details, such as symbol, price, and quantity. Bots should populate this with valid `OrderRequest` structs to execute multiple orders efficiently.
    pub requests: Vec<OrderRequest<'a>>,
}

impl<'a> BatchPlaceRequest<'a> {
    /// Constructs a new BatchPlace request with specified parameters.
    ///
    /// Allows customization of the batch order request. Bots should use this to define the category and list of orders to place.
    pub fn new(category: Category, requests: Vec<OrderRequest<'a>>) -> BatchPlaceRequest<'a> {
        BatchPlaceRequest { category, requests }
    }
}
