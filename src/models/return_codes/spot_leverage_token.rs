use crate::prelude::*;

/// Enum representing Bybit API V5 Spot Leverage Token error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#spot-leverage-token
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum SpotLeverageTokenCode {
    /// Leverage token is not supported.
    LeverageTokenNotSupported = 131001,

    /// Leverage token purchase failed.
    PurchaseFailed = 131002,

    /// Leverage token redemption failed.
    RedemptionFailed = 131003,

    /// Insufficient balance in account.
    InsufficientBalance = 131004,

    /// Invalid purchase amount.
    PurchaseAmountInvalid = 131005,

    /// Invalid redemption amount.
    RedemptionAmountInvalid = 131006,

    /// Leverage token is suspended.
    LeverageTokenSuspended = 131007,

    /// Leverage token purchase is suspended.
    PurchaseSuspended = 131008,

    /// Leverage token redemption is suspended.
    RedemptionSuspended = 131009,
}

impl SpotLeverageTokenCode {
    /// Converts an error code (as i32 or &str) to a SpotLeverageTokenError variant.
    /// Returns None if the code doesn't match any variant.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            131001 => Some(Self::LeverageTokenNotSupported),
            131002 => Some(Self::PurchaseFailed),
            131003 => Some(Self::RedemptionFailed),
            131004 => Some(Self::InsufficientBalance),
            131005 => Some(Self::PurchaseAmountInvalid),
            131006 => Some(Self::RedemptionAmountInvalid),
            131007 => Some(Self::LeverageTokenSuspended),
            131008 => Some(Self::PurchaseSuspended),
            131009 => Some(Self::RedemptionSuspended),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::LeverageTokenNotSupported => "Leverage token is not supported.",
            Self::PurchaseFailed => "Leverage token purchase failed.",
            Self::RedemptionFailed => "Leverage token redemption failed.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::PurchaseAmountInvalid => "Invalid purchase amount.",
            Self::RedemptionAmountInvalid => "Invalid redemption amount.",
            Self::LeverageTokenSuspended => "Leverage token is suspended.",
            Self::PurchaseSuspended => "Leverage token purchase is suspended.",
            Self::RedemptionSuspended => "Leverage token redemption is suspended.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = SpotLeverageTokenCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(131001), Some(Sut::LeverageTokenNotSupported));
        assert_eq!(Sut::from_code(131009), Some(Sut::RedemptionSuspended));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::LeverageTokenNotSupported;
        assert_eq!(
            error.to_string(),
            "131001 - Leverage token is not supported."
        );
        let error = Sut::RedemptionSuspended;
        assert_eq!(
            error.to_string(),
            "131009 - Leverage token redemption is suspended."
        );
    }
}
