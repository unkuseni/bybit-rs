use crate::prelude::*;

/// Parameters for moving a position between accounts.
///
/// Used to construct a request to the `/v5/position/move-position` endpoint to transfer a position from one Bybit account (UID) to another. Bots use this for portfolio management, such as consolidating positions or transferring risk between accounts in perpetual futures trading.
#[derive(Clone, Default, Serialize)]
pub struct MovePositionRequest<'a> {
    /// The source account UID.
    ///
    /// The unique identifier of the account from which the position is being transferred. Bots must ensure this UID has sufficient permissions and position size.
    pub from_uid: u64,
    /// The destination account UID.
    ///
    /// The unique identifier of the account to which the position is being transferred. Bots must verify the destination account is eligible to receive the position.
    pub to_uid: u64,
    /// A list of positions to move.
    ///
    /// Contains the details of each position to transfer, including symbol, price, side, and quantity. Bots should populate this with valid `PositionItem` structs to specify the exact positions to move.
    pub list: Vec<PositionItem<'a>>,
}
