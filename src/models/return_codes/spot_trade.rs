use crate::prelude::*;

/// Enum representing Bybit API V5 Spot Trade error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#spot-trade
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum SpotTradeCode {
    /// Order does not exist.
    OrderNotExist = 130001,

    /// Order already cancelled.
    OrderAlreadyCancelled = 130002,

    /// Order already filled.
    OrderAlreadyFilled = 130003,

    /// Insufficient balance in account.
    InsufficientBalance = 130004,

    /// Invalid price.
    PriceInvalid = 130005,

    /// Invalid order quantity.
    OrderQtyInvalid = 130006,

    /// Invalid order type.
    OrderTypeInvalid = 130007,

    /// Invalid order side.
    OrderSideInvalid = 130008,

    /// Invalid time-in-force.
    TimeInForceInvalid = 130009,

    /// Symbol not found.
    SymbolNotFound = 130010,

    /// Order creation failed.
    OrderCreationFailed = 130011,

    /// Order cancellation failed.
    OrderCancellationFailed = 130012,

    /// Order amend failed.
    OrderAmendFailed = 130013,

    /// Invalid order amount.
    OrderAmountInvalid = 130014,

    /// Spot trading is not supported for this symbol.
    SpotTradingNotSupported = 130015,

    /// Trigger price invalid.
    TriggerPriceInvalid = 130016,

    /// Invalid trigger price type.
    TriggerPriceTypeInvalid = 130017,

    /// Order price deviates significantly from market price.
    OrderPriceDeviate = 130018,

    /// Order creation timeout.
    OrderCreationTimeout = 130019,

    /// Invalid account type.
    InvalidAccountType = 130020,

    /// Spot trading is disabled for this sub-account.
    SpotTradingDisabledSubAccount = 130021,

    /// Spot trading is disabled for this master account.
    SpotTradingDisabledMasterAccount = 130022,

    /// Only Good-Till-Canceled (GTC) orders supported during Call Auction.
    GtcOnlyCallAuction = 130099,
}

impl SpotTradeCode {
    /// Converts an error code (as i32 or &str) to a SpotTradeError variant.
    /// Returns None if the code doesn't match any variant.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            130001 => Some(Self::OrderNotExist),
            130002 => Some(Self::OrderAlreadyCancelled),
            130003 => Some(Self::OrderAlreadyFilled),
            130004 => Some(Self::InsufficientBalance),
            130005 => Some(Self::PriceInvalid),
            130006 => Some(Self::OrderQtyInvalid),
            130007 => Some(Self::OrderTypeInvalid),
            130008 => Some(Self::OrderSideInvalid),
            130009 => Some(Self::TimeInForceInvalid),
            130010 => Some(Self::SymbolNotFound),
            130011 => Some(Self::OrderCreationFailed),
            130012 => Some(Self::OrderCancellationFailed),
            130013 => Some(Self::OrderAmendFailed),
            130014 => Some(Self::OrderAmountInvalid),
            130015 => Some(Self::SpotTradingNotSupported),
            130016 => Some(Self::TriggerPriceInvalid),
            130017 => Some(Self::TriggerPriceTypeInvalid),
            130018 => Some(Self::OrderPriceDeviate),
            130019 => Some(Self::OrderCreationTimeout),
            130020 => Some(Self::InvalidAccountType),
            130021 => Some(Self::SpotTradingDisabledSubAccount),
            130022 => Some(Self::SpotTradingDisabledMasterAccount),
            130099 => Some(Self::GtcOnlyCallAuction),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::OrderNotExist => "Order does not exist.",
            Self::OrderAlreadyCancelled => "Order already cancelled.",
            Self::OrderAlreadyFilled => "Order already filled.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::PriceInvalid => "Invalid price.",
            Self::OrderQtyInvalid => "Invalid order quantity.",
            Self::OrderTypeInvalid => "Invalid order type.",
            Self::OrderSideInvalid => "Invalid order side.",
            Self::TimeInForceInvalid => "Invalid time-in-force.",
            Self::SymbolNotFound => "Symbol not found.",
            Self::OrderCreationFailed => "Order creation failed.",
            Self::OrderCancellationFailed => "Order cancellation failed.",
            Self::OrderAmendFailed => "Order amend failed.",
            Self::OrderAmountInvalid => "Invalid order amount.",
            Self::SpotTradingNotSupported => "Spot trading is not supported for this symbol.",
            Self::TriggerPriceInvalid => "Trigger price invalid.",
            Self::TriggerPriceTypeInvalid => "Invalid trigger price type.",
            Self::OrderPriceDeviate => "Order price deviates significantly from market price.",
            Self::OrderCreationTimeout => "Order creation timeout.",
            Self::InvalidAccountType => "Invalid account type.",
            Self::SpotTradingDisabledSubAccount => "Spot trading is disabled for this sub-account.",
            Self::SpotTradingDisabledMasterAccount => {
                "Spot trading is disabled for this master account."
            }
            Self::GtcOnlyCallAuction => {
                "Only Good-Till-Canceled (GTC) orders supported during Call Auction."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = SpotTradeCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(130001), Some(Sut::OrderNotExist));
        assert_eq!(Sut::from_code(130099), Some(Sut::GtcOnlyCallAuction));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::OrderNotExist;
        assert_eq!(error.to_string(), "130001 - Order does not exist.");
        let error = Sut::GtcOnlyCallAuction;
        assert_eq!(
            error.to_string(),
            "130099 - Only Good-Till-Canceled (GTC) orders supported during Call Auction."
        );
    }
}
