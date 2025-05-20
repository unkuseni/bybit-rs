use crate::prelude::*;

/// Represents an options instrument.
///
/// Contains details about an options contract. Not relevant for perpetual futures but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInstrument {
    /// The trading pair symbol (e.g., "BTC-30DEC22-50000-C").
    ///
    /// Identifies the options contract. Not relevant for perpetuals.
    pub symbol: String,

    /// The trading status (e.g., "Trading").
    ///
    /// Indicates if the options contract is tradable. Not relevant for perpetuals.
    pub status: String,

    /// The base coin (e.g., "BTC").
    ///
    /// The underlying asset of the options contract. Not relevant for perpetuals.
    pub base_coin: String,

    /// The quote coin (e.g., "USDT").
    ///
    /// The currency used to quote the options contract. Not relevant for perpetuals.
    pub quote_coin: String,

    /// The settlement coin (e.g., "USDT").
    ///
    /// The currency used for settlement. Not relevant for perpetuals.
    pub settle_coin: String,

    /// The type of option (e.g., "Call", "Put").
    ///
    /// Specifies whether the option is a call or put. Not relevant for perpetuals.
    pub option_type: String,

    /// The launch time of the options contract (Unix timestamp in milliseconds).
    ///
    /// Indicates when the contract was listed. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub launch_time: u64,

    /// The delivery time of the options contract (Unix timestamp in milliseconds).
    ///
    /// Indicates the expiry date of the option. Not relevant for perpetuals.
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,

    /// The delivery fee rate for the options contract.
    ///
    /// Specifies the fee for settlement. Not relevant for perpetuals.
    pub delivery_fee_rate: String,

    /// The price constraints.
    ///
    /// Defines price constraints for options trading. Not relevant for perpetuals.
    pub price_filter: PriceFilter,

    /// The lot size constraints.
    ///
    /// Defines order quantity constraints for options trading. Not relevant for perpetuals.
    pub lot_size_filter: LotSizeFilter,
}
