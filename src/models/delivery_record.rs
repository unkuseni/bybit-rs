use crate::prelude::*;
/// Query delivery records of Invese Futures, USDC Futures, USDT Futures and Options, sorted by deliveryTime in descending order
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecord {
    /// Symbol of the delivery record
    pub symbol: String,

    /// Side of the delivery record
    pub side: Side,

    /// Delivery time of the delivery record
    pub delivery_time: u64,

    /// Strike price of the delivery record
    #[serde(with = "string_to_float")]
    pub strike: f64,

    /// Fee of the delivery record
    #[serde(with = "string_to_float")]
    pub fee: f64,

    /// Position of the delivery record
    #[serde(with = "string_to_float")]
    pub position: f64,

    /// Delivery price of the delivery record
    #[serde(with = "string_to_float")]
    pub delivery_price: f64,

    /// Delivery realized profit and loss of the delivery record
    #[serde(with = "string_to_float")]
    pub delivery_rpl: f64,
}
