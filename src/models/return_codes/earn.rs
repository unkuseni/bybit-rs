use crate::Display;

/// Enum representing Bybit API V5 Earn error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#earn
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum EarnCode {
    /// Earn product is not supported.
    EarnProductNotSupported = 160001,

    /// Insufficient balance in account.
    InsufficientBalance = 160002,

    /// Invalid subscription amount.
    InvalidSubscriptionAmount = 160003,

    /// Subscription failed.
    SubscriptionFailed = 160004,

    /// Redemption failed.
    RedemptionFailed = 160005,

    /// Invalid redemption amount.
    InvalidRedemptionAmount = 160006,

    /// Earn product is disabled for this account.
    EarnProductDisabled = 160007,

    /// Earn product order does not exist.
    EarnOrderNotExist = 160008,
}

impl EarnCode {
    /// Converts an error code (as i32) to an EarnError variant.
    /// Returns None if the code doesn't match any variant or if the string cannot be parsed.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            160001 => Some(Self::EarnProductNotSupported),
            160002 => Some(Self::InsufficientBalance),
            160003 => Some(Self::InvalidSubscriptionAmount),
            160004 => Some(Self::SubscriptionFailed),
            160005 => Some(Self::RedemptionFailed),
            160006 => Some(Self::InvalidRedemptionAmount),
            160007 => Some(Self::EarnProductDisabled),
            160008 => Some(Self::EarnOrderNotExist),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::EarnProductNotSupported => "Earn product is not supported.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::InvalidSubscriptionAmount => "Invalid subscription amount.",
            Self::SubscriptionFailed => "Subscription failed.",
            Self::RedemptionFailed => "Redemption failed.",
            Self::InvalidRedemptionAmount => "Invalid redemption amount.",
            Self::EarnProductDisabled => "Earn product is disabled for this account.",
            Self::EarnOrderNotExist => "Earn product order does not exist.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = EarnCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(160001), Some(Sut::EarnProductNotSupported));
        assert_eq!(Sut::from_code(160008), Some(Sut::EarnOrderNotExist));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::EarnProductNotSupported;
        assert_eq!(error.to_string(), "160001 - Earn product is not supported.");
        let error = Sut::EarnOrderNotExist;
        assert_eq!(
            error.to_string(),
            "160008 - Earn product order does not exist."
        );
    }
}
