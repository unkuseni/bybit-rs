use crate::prelude::*;

/// Enum representing Bybit API V5 UTA error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#uta
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum UtaCode {
    /// Invalid parameter(s).
    InvalidParameters = 10001,

    /// Invalid request: server error or request timeout.
    InvalidRequest = 10002,

    /// API key is invalid.
    InvalidApiKey = 10003,

    /// Signature error.
    SignatureError = 10004,

    /// Permission denied.
    PermissionDenied = 10005,

    /// Too many visits, exceeding IP limit.
    TooManyVisits = 10006,

    /// Unmatched IP: API key not bound to this IP.
    UnmatchedIp = 10010,

    /// Order does not exist.
    OrderNotExist = 110001,

    /// Insufficient balance in account.
    InsufficientBalance = 110002,

    /// Position does not exist.
    PositionNotExist = 110003,

    /// Insufficient margin in account.
    InsufficientMargin = 110004,

    /// Insufficient position size.
    InsufficientPositionSize = 110005,

    /// Position mode error: cannot change position mode when position exists.
    PositionModeError = 110006,

    /// Symbol not found.
    SymbolNotFound = 110007,

    /// Currency not supported.
    CurrencyNotSupported = 110008,

    /// Invalid stop order.
    StopOrderInvalid = 110009,

    /// Invalid price.
    PriceInvalid = 110010,

    /// Order quantity invalid.
    OrderQtyInvalid = 110011,

    /// Invalid order type.
    OrderTypeInvalid = 110012,

    /// Leverage not modified.
    LeverageNotModified = 110013,

    /// Leverage setting error.
    LeverageSettingError = 110014,

    /// Invalid take profit/stop loss setting.
    TpSlSettingInvalid = 110015,

    /// Order price deviates significantly from market price.
    OrderPriceDeviate = 110017,

    /// Order already cancelled.
    OrderAlreadyCancelled = 110019,

    /// Order already filled.
    OrderAlreadyFilled = 110020,

    /// Trigger price invalid.
    TriggerPriceInvalid = 110021,

    /// Reduce-only error.
    ReduceOnlyError = 110022,

    /// Invalid time-in-force.
    TimeInForceInvalid = 110023,

    /// Invalid order side.
    OrderSideInvalid = 110024,

    /// Invalid position mode.
    PositionModeInvalid = 110025,

    /// Order price is lower than liquidation price.
    PriceLowerThanLiquidation = 110026,

    /// No need to change margin.
    MarginChangeNotNeeded = 110027,

    /// Order creation timeout.
    OrderCreationTimeout = 110028,

    /// Order type not supported.
    OrderTypeNotSupported = 110029,

    /// Invalid trigger price type.
    TriggerPriceTypeInvalid = 110030,

    /// Invalid stop loss price.
    StopLossPriceInvalid = 110031,

    /// Invalid take profit price.
    TakeProfitPriceInvalid = 110032,

    /// Stop loss/take profit quantity invalid.
    TpSlQtyInvalid = 110033,

    /// Stop loss/take profit price type invalid.
    TpSlPriceTypeInvalid = 110034,

    /// Order creation failed.
    OrderCreationFailed = 110035,

    /// Order cancellation failed.
    OrderCancellationFailed = 110036,

    /// Order amend failed.
    OrderAmendFailed = 110037,

    /// Risk limit not modified.
    RiskLimitNotModified = 110038,

    /// Invalid risk ID.
    RiskIdInvalid = 110039,

    /// Non-UTA users cannot access Pre-Market Perpetual Trading.
    NonUtaPreMarket = 110097,

    /// Only Good-Till-Canceled (GTC) orders supported during Call Auction.
    GtcOnlyCallAuction = 110098,
}

impl UtaCode {
    /// Converts an error code (as i32 or &str) to a UtaError variant.
    /// Returns None if the code doesn't match any variant.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            10001 => Some(Self::InvalidParameters),
            10002 => Some(Self::InvalidRequest),
            10003 => Some(Self::InvalidApiKey),
            10004 => Some(Self::SignatureError),
            10005 => Some(Self::PermissionDenied),
            10006 => Some(Self::TooManyVisits),
            10010 => Some(Self::UnmatchedIp),
            110001 => Some(Self::OrderNotExist),
            110002 => Some(Self::InsufficientBalance),
            110003 => Some(Self::PositionNotExist),
            110004 => Some(Self::InsufficientMargin),
            110005 => Some(Self::InsufficientPositionSize),
            110006 => Some(Self::PositionModeError),
            110007 => Some(Self::SymbolNotFound),
            110008 => Some(Self::CurrencyNotSupported),
            110009 => Some(Self::StopOrderInvalid),
            110010 => Some(Self::PriceInvalid),
            110011 => Some(Self::OrderQtyInvalid),
            110012 => Some(Self::OrderTypeInvalid),
            110013 => Some(Self::LeverageNotModified),
            110014 => Some(Self::LeverageSettingError),
            110015 => Some(Self::TpSlSettingInvalid),
            110017 => Some(Self::OrderPriceDeviate),
            110019 => Some(Self::OrderAlreadyCancelled),
            110020 => Some(Self::OrderAlreadyFilled),
            110021 => Some(Self::TriggerPriceInvalid),
            110022 => Some(Self::ReduceOnlyError),
            110023 => Some(Self::TimeInForceInvalid),
            110024 => Some(Self::OrderSideInvalid),
            110025 => Some(Self::PositionModeInvalid),
            110026 => Some(Self::PriceLowerThanLiquidation),
            110027 => Some(Self::MarginChangeNotNeeded),
            110028 => Some(Self::OrderCreationTimeout),
            110029 => Some(Self::OrderTypeNotSupported),
            110030 => Some(Self::TriggerPriceTypeInvalid),
            110031 => Some(Self::StopLossPriceInvalid),
            110032 => Some(Self::TakeProfitPriceInvalid),
            110033 => Some(Self::TpSlQtyInvalid),
            110034 => Some(Self::TpSlPriceTypeInvalid),
            110035 => Some(Self::OrderCreationFailed),
            110036 => Some(Self::OrderCancellationFailed),
            110037 => Some(Self::OrderAmendFailed),
            110038 => Some(Self::RiskLimitNotModified),
            110039 => Some(Self::RiskIdInvalid),
            110097 => Some(Self::NonUtaPreMarket),
            110098 => Some(Self::GtcOnlyCallAuction),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidParameters => "Invalid parameter(s).",
            Self::InvalidRequest => "Invalid request: server error or request timeout.",
            Self::InvalidApiKey => "API key is invalid.",
            Self::SignatureError => "Signature error.",
            Self::PermissionDenied => "Permission denied.",
            Self::TooManyVisits => "Too many visits, exceeding IP limit.",
            Self::UnmatchedIp => "Unmatched IP: API key not bound to this IP.",
            Self::OrderNotExist => "Order does not exist.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::PositionNotExist => "Position does not exist.",
            Self::InsufficientMargin => "Insufficient margin in account.",
            Self::InsufficientPositionSize => "Insufficient position size.",
            Self::PositionModeError => {
                "Position mode error: cannot change position mode when position exists."
            }
            Self::SymbolNotFound => "Symbol not found.",
            Self::CurrencyNotSupported => "Currency not supported.",
            Self::StopOrderInvalid => "Invalid stop order.",
            Self::PriceInvalid => "Invalid price.",
            Self::OrderQtyInvalid => "Order quantity invalid.",
            Self::OrderTypeInvalid => "Invalid order type.",
            Self::LeverageNotModified => "Leverage not modified.",
            Self::LeverageSettingError => "Leverage setting error.",
            Self::TpSlSettingInvalid => "Invalid take profit/stop loss setting.",
            Self::OrderPriceDeviate => "Order price deviates significantly from market price.",
            Self::OrderAlreadyCancelled => "Order already cancelled.",
            Self::OrderAlreadyFilled => "Order already filled.",
            Self::TriggerPriceInvalid => "Trigger price invalid.",
            Self::ReduceOnlyError => "Reduce-only error.",
            Self::TimeInForceInvalid => "Invalid time-in-force.",
            Self::OrderSideInvalid => "Invalid order side.",
            Self::PositionModeInvalid => "Invalid position mode.",
            Self::PriceLowerThanLiquidation => "Order price is lower than liquidation price.",
            Self::MarginChangeNotNeeded => "No need to change margin.",
            Self::OrderCreationTimeout => "Order creation timeout.",
            Self::OrderTypeNotSupported => "Order type not supported.",
            Self::TriggerPriceTypeInvalid => "Invalid trigger price type.",
            Self::StopLossPriceInvalid => "Invalid stop loss price.",
            Self::TakeProfitPriceInvalid => "Invalid take profit price.",
            Self::TpSlQtyInvalid => "Stop loss/take profit quantity invalid.",
            Self::TpSlPriceTypeInvalid => "Stop loss/take profit price type invalid.",
            Self::OrderCreationFailed => "Order creation failed.",
            Self::OrderCancellationFailed => "Order cancellation failed.",
            Self::OrderAmendFailed => "Order amend failed.",
            Self::RiskLimitNotModified => "Risk limit not modified.",
            Self::RiskIdInvalid => "Invalid risk ID.",
            Self::NonUtaPreMarket => "Non-UTA users cannot access Pre-Market Perpetual Trading.",
            Self::GtcOnlyCallAuction => {
                "Only Good-Till-Canceled (GTC) orders supported during Call Auction."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = UtaCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(10001), Some(Sut::InvalidParameters));
        assert_eq!(Sut::from_code(110097), Some(Sut::NonUtaPreMarket));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_message() {
        assert_eq!(Sut::InvalidParameters.message(), "Invalid parameter(s).");
        assert_eq!(
            Sut::NonUtaPreMarket.message(),
            "Non-UTA users cannot access Pre-Market Perpetual Trading."
        );
    }
}
