use crate::prelude::*;

/// Enum representing Bybit API V5 Crypto Loan return error/codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#crypto-loan
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum CryptoLoanCode {
    /// Crypto loan is not supported.
    CryptoLoanNotSupported = 150001,

    /// Insufficient balance in account.
    InsufficientBalance = 150002,

    /// Invalid loan amount.
    InvalidLoanAmount = 150003,

    /// Loan failed.
    LoanFailed = 150004,

    /// Repay failed.
    RepayFailed = 150005,

    /// Invalid repay amount.
    InvalidRepayAmount = 150006,

    /// Crypto loan is disabled for this account.
    CryptoLoanDisabled = 150007,

    /// Crypto loan order does not exist.
    LoanOrderNotExist = 150008,
}

impl CryptoLoanCode {
    /// Converts an error code (as i32 or &str) to a CryptoLoanError variant.
    /// Returns None if the code doesn't match any variant or if the string cannot be parsed.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            150001 => Some(Self::CryptoLoanNotSupported),
            150002 => Some(Self::InsufficientBalance),
            150003 => Some(Self::InvalidLoanAmount),
            150004 => Some(Self::LoanFailed),
            150005 => Some(Self::RepayFailed),
            150006 => Some(Self::InvalidRepayAmount),
            150007 => Some(Self::CryptoLoanDisabled),
            150008 => Some(Self::LoanOrderNotExist),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::CryptoLoanNotSupported => "Crypto loan is not supported.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::InvalidLoanAmount => "Invalid loan amount.",
            Self::LoanFailed => "Loan failed.",
            Self::RepayFailed => "Repay failed.",
            Self::InvalidRepayAmount => "Invalid repay amount.",
            Self::CryptoLoanDisabled => "Crypto loan is disabled for this account.",
            Self::LoanOrderNotExist => "Crypto loan order does not exist.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = CryptoLoanCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(150001), Some(Sut::CryptoLoanNotSupported));
        assert_eq!(Sut::from_code(150008), Some(Sut::LoanOrderNotExist));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::CryptoLoanNotSupported;
        assert_eq!(error.to_string(), "150001 - Crypto loan is not supported.");
        let error = Sut::LoanOrderNotExist;
        assert_eq!(
            error.to_string(),
            "150008 - Crypto loan order does not exist."
        );
    }
}
