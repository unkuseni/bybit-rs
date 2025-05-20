use crate::prelude::*;

/// Enum representing different types of batch order requests.
///
/// Allows bots to specify whether a batch request is for creating, amending, or canceling orders. This provides flexibility for handling multiple order operations in a unified way.
#[derive(Clone)]
pub enum RequestType<'a> {
    /// A batch order creation request.
    ///
    /// Used to place multiple new orders. Bots use this for strategies requiring simultaneous order placements, such as grid trading.
    Create(BatchPlaceRequest<'a>),

    /// A batch order amendment request.
    ///
    /// Used to modify multiple existing orders. Bots use this to adjust order parameters in response to market changes.
    Amend(BatchAmendRequest<'a>),

    /// A batch order cancellation request.
    ///
    /// Used to cancel multiple existing orders. Bots use this for risk management or strategy adjustments.
    Cancel(BatchCancelRequest<'a>),
}
