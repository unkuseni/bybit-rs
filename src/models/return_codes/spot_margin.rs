use crate::prelude::*;

/// Enum representing Bybit API V5 Spot Margin error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#spot-margin-trade
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum SpotMarginCode {
    /// Spot margin trading is not enabled.
    MarginTradingNotEnabled = 132001,

    /// Spot margin trading is disabled for this sub-account.
    MarginTradingDisabledSubAccount = 132002,

    /// Spot margin trading is disabled for this master account.
    MarginTradingDisabledMasterAccount = 132003,

    /// Insufficient margin balance.
    InsufficientMarginBalance = 132004,

    /// Invalid leverage.
    InvalidLeverage = 132005,

    /// Invalid borrow amount.
    InvalidBorrowAmount = 132006,

    /// Borrow failed.
    BorrowFailed = 132007,

    /// Repay failed.
    RepayFailed = 132008,

    /// Invalid repay amount.
    InvalidRepayAmount = 132009,

    /// Margin level is too low.
    MarginLevelTooLow = 132010,

    /// Spot margin trading is not supported for this symbol.
    MarginTradingNotSupported = 132011,

    /// Invalid margin account type.
    InvalidMarginAccountType = 132012,
}

impl SpotMarginCode {
    /// Converts an error code (as i32 or &str) to a SpotMarginError variant.
    /// Returns None if the code doesn't match any variant.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            132001 => Some(Self::MarginTradingNotEnabled),
            132002 => Some(Self::MarginTradingDisabledSubAccount),
            132003 => Some(Self::MarginTradingDisabledMasterAccount),
            132004 => Some(Self::InsufficientMarginBalance),
            132005 => Some(Self::InvalidLeverage),
            132006 => Some(Self::InvalidBorrowAmount),
            132007 => Some(Self::BorrowFailed),
            132008 => Some(Self::RepayFailed),
            132009 => Some(Self::InvalidRepayAmount),
            132010 => Some(Self::MarginLevelTooLow),
            132011 => Some(Self::MarginTradingNotSupported),
            132012 => Some(Self::InvalidMarginAccountType),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::MarginTradingNotEnabled => "Spot margin trading is not enabled.",
            Self::MarginTradingDisabledSubAccount => {
                "Spot margin trading is disabled for this sub-account."
            }
            Self::MarginTradingDisabledMasterAccount => {
                "Spot margin trading is disabled for this master account."
            }
            Self::InsufficientMarginBalance => "Insufficient margin balance.",
            Self::InvalidLeverage => "Invalid leverage.",
            Self::InvalidBorrowAmount => "Invalid borrow amount.",
            Self::BorrowFailed => "Borrow failed.",
            Self::RepayFailed => "Repay failed.",
            Self::InvalidRepayAmount => "Invalid repay amount.",
            Self::MarginLevelTooLow => "Margin level is too low.",
            Self::MarginTradingNotSupported => {
                "Spot margin trading is not supported for this symbol."
            }
            Self::InvalidMarginAccountType => "Invalid margin account type.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = SpotMarginCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(132001), Some(Sut::MarginTradingNotEnabled));
        assert_eq!(Sut::from_code(132012), Some(Sut::InvalidMarginAccountType));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::MarginTradingNotEnabled;
        assert_eq!(
            error.to_string(),
            "132001 - Spot margin trading is not enabled."
        );
        let error = Sut::InvalidMarginAccountType;
        assert_eq!(error.to_string(), "132012 - Invalid margin account type.");
    }
}
