use crate::prelude::*;

/// Specifies the time-in-force policy for an order, controlling how long it remains active.
///
/// Time-in-force (TIF) dictates the duration an order stays open before it is executed
/// or canceled. In perpetual futures trading, TIF is critical for managing order
/// execution in volatile markets. For example:
/// - **GTC (Good Till Canceled)**: Remains active until filled or manually canceled, suitable for patient strategies.
/// - **IOC (Immediate or Cancel)**: Executes immediately, with any unfilled portion canceled, ideal for high-frequency trading.
/// - **FOK (Fill or Kill)**: Must be fully filled immediately or canceled, used for large orders needing guaranteed execution.
/// - **PostOnly**: Ensures the order adds liquidity (maker order) and cancels if it would take liquidity, useful for fee optimization.
///
/// Bots must carefully select TIF based on strategy. For instance, a PostOnly order
/// can reduce fees but may fail in fast-moving markets, impacting profitability.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TimeInForce {
    /// Good Till Canceled: Order remains active until filled or canceled.
    #[default]
    GTC,

    /// Immediate or Cancel: Executes immediately, with unfilled portion canceled.
    IOC,

    /// Fill or Kill: Must be fully filled immediately or canceled.
    FOK,

    /// Post Only: Ensures the order is a maker order, cancels if it would be a taker.
    PostOnly,
}

impl TimeInForce {
    /// Converts the `TimeInForce` enum to its string representation.
    ///
    /// Returns the string format expected by Bybit's API, such as "GTC" or "PostOnly".
    /// Bots should use this method to ensure API compatibility when setting TIF.
    pub fn as_str(&self) -> &str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        }
    }
}
