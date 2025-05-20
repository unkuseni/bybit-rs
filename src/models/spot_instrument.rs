use crate::prelude::*;

/// Represents a single spot instrument.
///
/// Contains details about a spot trading pair. Not relevant for perpetual futures but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrument {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the spot trading pair. Bots trading perpetuals can ignore this.
    pub symbol: String,

    /// The base coin (e.g., "BTC").
    ///
    /// The underlying asset of the spot pair. Not relevant for perpetuals.
    pub base_coin: String,

    /// The quote coin (e.g., "USDT").
    ///
    /// The currency used to quote the spot pair. Not relevant for perpetuals.
    pub quote_coin: String,

    /// Indicates if the pair is an innovation token (0 or 1).
    ///
    /// Marks new or experimental tokens. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub innovation: u64,

    /// The trading status (e.g., "Trading").
    ///
    /// Indicates if the spot pair is tradable. Not relevant for perpetuals.
    pub status: String,

    /// Indicates if margin trading is supported.
    ///
    /// Specifies whether the spot pair supports margin trading. Not relevant for perpetuals.
    pub margin_trading: String,

    /// A tag for the spot pair (e.g., for special tokens).
    ///
    /// Used for categorization. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub st_tag: u64,

    /// The lot size constraints.
    ///
    /// Defines order quantity constraints for spot trading. Not relevant for perpetuals.
    pub lot_size_filter: LotSizeFilter,

    /// The price constraints.
    ///
    /// Defines price constraints for spot trading. Not relevant for perpetuals.
    pub price_filter: PriceFilter,

    /// Risk parameters for the spot pair.
    ///
    /// Specifies risk-related constraints. Not relevant for perpetuals.
    pub risk_parameters: RiskParameters,
}
