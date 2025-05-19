use crate::prelude::*;

/// Parameters for batch canceling multiple orders.
///
/// Used to construct a request to the `/v5/order/cancel-batch` endpoint to cancel multiple existing orders simultaneously. This is useful for bots implementing risk management or adjusting strategies in perpetual futures.
#[derive(Clone, Default)]
pub struct BatchCancelRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to target the correct contract type for the batch cancellations.
    pub category: Category,
    /// A list of order cancellation requests.
    ///
    /// Contains the individual cancellation details, such as order ID or order link ID. Bots should populate this with valid `CancelOrderRequest` structs to cancel multiple orders efficiently.
    pub requests: Vec<CancelOrderRequest<'a>>,
}

impl<'a> BatchCancelRequest<'a> {
    /// Constructs a new BatchCancel request with specified parameters.
    ///
    /// Allows customization of the batch cancellation request. Bots should use this to define the category and list of cancellations to apply.
    pub fn new(
        category: Category,
        requests: Vec<CancelOrderRequest<'a>>,
    ) -> BatchCancelRequest<'a> {
        BatchCancelRequest { category, requests }
    }
}
