use crate::prelude::*;

/// Enum representing Bybit API V5 Asset return error/codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#asset
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum AssetCode {
    /// Insufficient balance in account.
    InsufficientBalance = 140001,

    /// Invalid coin.
    InvalidCoin = 140002,

    /// Invalid amount.
    InvalidAmount = 140003,

    /// Transfer failed.
    TransferFailed = 140004,

    /// Withdrawal failed.
    WithdrawalFailed = 140005,

    /// Deposit failed.
    DepositFailed = 140006,

    /// Coin does not exist.
    CoinNotExist = 140007,

    /// Withdrawal address is invalid.
    InvalidWithdrawalAddress = 140008,

    /// Withdrawal is disabled.
    WithdrawalDisabled = 140009,

    /// Deposit is disabled.
    DepositDisabled = 140010,

    /// Transfer is disabled.
    TransferDisabled = 140011,

    /// Withdrawal amount is too small.
    WithdrawalAmountTooSmall = 140012,

    /// Withdrawal amount is too large.
    WithdrawalAmountTooLarge = 140013,

    /// Withdrawal fee is too high.
    WithdrawalFeeTooHigh = 140014,

    /// Withdrawal address is not in whitelist.
    WithdrawalAddressNotWhitelisted = 140015,

    /// Withdrawal address is not supported.
    WithdrawalAddressNotSupported = 140016,

    /// Internal transfer is not allowed for this coin.
    InternalTransferNotAllowed = 140017,

    /// Withdrawal is not allowed for this account.
    WithdrawalNotAllowed = 140018,

    /// Deposit is not allowed for this account.
    DepositNotAllowed = 140019,

    /// Sub-account transfer is disabled.
    SubAccountTransferDisabled = 140020,

    /// Master account transfer is disabled.
    MasterAccountTransferDisabled = 140021,

    /// Invalid chain type.
    InvalidChainType = 140022,

    /// Withdrawal address is not on the selected chain.
    WithdrawalAddressWrongChain = 140023,

    /// Withdrawal is not supported for this coin.
    WithdrawalNotSupported = 140024,

    /// Deposit is not supported for this coin.
    DepositNotSupported = 140025,

    /// Transfer amount is too small.
    TransferAmountTooSmall = 140026,

    /// Transfer amount is too large.
    TransferAmountTooLarge = 140027,
}

impl AssetCode {
    /// Converts an error code (as i32 or &str) to an AssetError variant.
    /// Returns None if the code doesn't match any variant.
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        let code = code.into();
        match code {
            140001 => Some(Self::InsufficientBalance),
            140002 => Some(Self::InvalidCoin),
            140003 => Some(Self::InvalidAmount),
            140004 => Some(Self::TransferFailed),
            140005 => Some(Self::WithdrawalFailed),
            140006 => Some(Self::DepositFailed),
            140007 => Some(Self::CoinNotExist),
            140008 => Some(Self::InvalidWithdrawalAddress),
            140009 => Some(Self::WithdrawalDisabled),
            140010 => Some(Self::DepositDisabled),
            140011 => Some(Self::TransferDisabled),
            140012 => Some(Self::WithdrawalAmountTooSmall),
            140013 => Some(Self::WithdrawalAmountTooLarge),
            140014 => Some(Self::WithdrawalFeeTooHigh),
            140015 => Some(Self::WithdrawalAddressNotWhitelisted),
            140016 => Some(Self::WithdrawalAddressNotSupported),
            140017 => Some(Self::InternalTransferNotAllowed),
            140018 => Some(Self::WithdrawalNotAllowed),
            140019 => Some(Self::DepositNotAllowed),
            140020 => Some(Self::SubAccountTransferDisabled),
            140021 => Some(Self::MasterAccountTransferDisabled),
            140022 => Some(Self::InvalidChainType),
            140023 => Some(Self::WithdrawalAddressWrongChain),
            140024 => Some(Self::WithdrawalNotSupported),
            140025 => Some(Self::DepositNotSupported),
            140026 => Some(Self::TransferAmountTooSmall),
            140027 => Some(Self::TransferAmountTooLarge),
            _ => None,
        }
    }

    /// Returns the error message associated with the error code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::InvalidCoin => "Invalid coin.",
            Self::InvalidAmount => "Invalid amount.",
            Self::TransferFailed => "Transfer failed.",
            Self::WithdrawalFailed => "Withdrawal failed.",
            Self::DepositFailed => "Deposit failed.",
            Self::CoinNotExist => "Coin does not exist.",
            Self::InvalidWithdrawalAddress => "Withdrawal address is invalid.",
            Self::WithdrawalDisabled => "Withdrawal is disabled.",
            Self::DepositDisabled => "Deposit is disabled.",
            Self::TransferDisabled => "Transfer is disabled.",
            Self::WithdrawalAmountTooSmall => "Withdrawal amount is too small.",
            Self::WithdrawalAmountTooLarge => "Withdrawal amount is too large.",
            Self::WithdrawalFeeTooHigh => "Withdrawal fee is too high.",
            Self::WithdrawalAddressNotWhitelisted => "Withdrawal address is not in whitelist.",
            Self::WithdrawalAddressNotSupported => "Withdrawal address is not supported.",
            Self::InternalTransferNotAllowed => "Internal transfer is not allowed for this coin.",
            Self::WithdrawalNotAllowed => "Withdrawal is not allowed for this account.",
            Self::DepositNotAllowed => "Deposit is not allowed for this account.",
            Self::SubAccountTransferDisabled => "Sub-account transfer is disabled.",
            Self::MasterAccountTransferDisabled => "Master account transfer is disabled.",
            Self::InvalidChainType => "Invalid chain type.",
            Self::WithdrawalAddressWrongChain => "Withdrawal address is not on the selected chain.",
            Self::WithdrawalNotSupported => "Withdrawal is not supported for this coin.",
            Self::DepositNotSupported => "Deposit is not supported for this coin.",
            Self::TransferAmountTooSmall => "Transfer amount is too small.",
            Self::TransferAmountTooLarge => "Transfer amount is too large.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = AssetCode;
    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(140001), Some(Sut::InsufficientBalance));
        assert_eq!(Sut::from_code(140027), Some(Sut::TransferAmountTooLarge));
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::InsufficientBalance;
        assert_eq!(
            error.to_string(),
            "140001 - Insufficient balance in account."
        );
        let error = Sut::TransferAmountTooLarge;
        assert_eq!(error.to_string(), "140027 - Transfer amount is too large.");
    }
}
