use crate::prelude::*;

/// Parameters for batch amending multiple orders.
///
/// Used to construct a request to the `/v5/order/amend-batch` endpoint to modify multiple existing orders simultaneously. This is useful for bots adjusting order parameters (e.g., price or quantity) in response to market conditions in perpetual futures.
#[derive(Clone, Default)]
pub struct BatchAmendRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type for the batch amendments.
    pub category: Category,
    /// A list of order amendment requests.
    ///
    /// Contains the individual amendment details, such as order ID, new price, or quantity. Bots should populate this with valid `AmendOrderRequest` structs to modify multiple orders efficiently.
    pub requests: Vec<AmendOrderRequest<'a>>,
}

impl<'a> BatchAmendRequest<'a> {
    /// Constructs a new BatchAmend request with specified parameters.
    ///
    /// Allows customization of the batch amendment request. Bots should use this to define the category and list of amendments to apply.
    pub fn new(category: Category, requests: Vec<AmendOrderRequest<'a>>) -> BatchAmendRequest<'a> {
        BatchAmendRequest { category, requests }
    }
}
