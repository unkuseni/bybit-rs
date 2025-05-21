use crate::prelude::*;

/// Enum representing Bybit API V5 General return error/codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error#general-error-codes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{} - {}", *self as i32, self.message())]
pub enum GeneralCode {
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

    /// Request header `bybit-agent` is empty or invalid.
    InvalidAgentHeader = 10014,

    /// Timestamp is expired or there is a clock skew.
    TimestampError = 10017,

    /// Futures/Options symbol does not exist.
    SymbolNotExist = 10021,

    /// Symbol is not trading.
    SymbolNotTrading = 10022,

    /// Invalid JSON format.
    InvalidJsonFormat = 100400,

    /// Invalid category.
    InvalidCategory = 100401,

    /// Internal server error.
    InternalServerError = 100500,

    /// Invalid account type.
    InvalidAccountType = 100600,

    /// Insufficient balance in account.
    InsufficientBalance = 100601,

    /// Invalid sub-account name.
    InvalidSubAccountName = 100602,

    /// Sub-account does not exist.
    SubAccountNotExist = 100603,

    /// Master account does not exist.
    MasterAccountNotExist = 100604,

    /// Failed to create sub-account.
    SubAccountCreationFailed = 100605,

    /// Sub-account already exists.
    SubAccountAlreadyExists = 100606,

    /// Invalid API key permission.
    InvalidApiKeyPermission = 100607,

    /// Failed to update API key.
    ApiKeyUpdateFailed = 100608,

    /// API key is disabled.
    ApiKeyDisabled = 100609,

    /// Failed to delete API key.
    ApiKeyDeletionFailed = 100610,

    /// Invalid batch order request: too many orders or other issues.
    InvalidBatchOrder = 100611,

    /// Sub-account is frozen.
    SubAccountFrozen = 100612,

    /// Sub-account permission error.
    SubAccountPermissionError = 100613,

    /// Master account is frozen.
    MasterAccountFrozen = 100614,

    /// Master-sub relationship already exists.
    MasterSubRelationshipExists = 100615,
}

impl GeneralCode {
    /// Converts an error code (as i32) to a GeneralError variant.
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
            10014 => Some(Self::InvalidAgentHeader),
            10017 => Some(Self::TimestampError),
            10021 => Some(Self::SymbolNotExist),
            10022 => Some(Self::SymbolNotTrading),
            100400 => Some(Self::InvalidJsonFormat),
            100401 => Some(Self::InvalidCategory),
            100500 => Some(Self::InternalServerError),
            100600 => Some(Self::InvalidAccountType),
            100601 => Some(Self::InsufficientBalance),
            100602 => Some(Self::InvalidSubAccountName),
            100603 => Some(Self::SubAccountNotExist),
            100604 => Some(Self::MasterAccountNotExist),
            100605 => Some(Self::SubAccountCreationFailed),
            100606 => Some(Self::SubAccountAlreadyExists),
            100607 => Some(Self::InvalidApiKeyPermission),
            100608 => Some(Self::ApiKeyUpdateFailed),
            100609 => Some(Self::ApiKeyDisabled),
            100610 => Some(Self::ApiKeyDeletionFailed),
            100611 => Some(Self::InvalidBatchOrder),
            100612 => Some(Self::SubAccountFrozen),
            100613 => Some(Self::SubAccountPermissionError),
            100614 => Some(Self::MasterAccountFrozen),
            100615 => Some(Self::MasterSubRelationshipExists),
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
            Self::InvalidAgentHeader => "Request header `bybit-agent` is empty or invalid.",
            Self::TimestampError => "Timestamp is expired or there is a clock skew.",
            Self::SymbolNotExist => "Futures/Options symbol does not exist.",
            Self::SymbolNotTrading => "Symbol is not trading.",
            Self::InvalidJsonFormat => "Invalid JSON format.",
            Self::InvalidCategory => "Invalid category.",
            Self::InternalServerError => "Internal server error.",
            Self::InvalidAccountType => "Invalid account type.",
            Self::InsufficientBalance => "Insufficient balance in account.",
            Self::InvalidSubAccountName => "Invalid sub-account name.",
            Self::SubAccountNotExist => "Sub-account does not exist.",
            Self::MasterAccountNotExist => "Master account does not exist.",
            Self::SubAccountCreationFailed => "Failed to create sub-account.",
            Self::SubAccountAlreadyExists => "Sub-account already exists.",
            Self::InvalidApiKeyPermission => "Invalid API key permission.",
            Self::ApiKeyUpdateFailed => "Failed to update API key.",
            Self::ApiKeyDisabled => "API key is disabled.",
            Self::ApiKeyDeletionFailed => "Failed to delete API key.",
            Self::InvalidBatchOrder => {
                "Invalid batch order request: too many orders or other issues."
            }
            Self::SubAccountFrozen => "Sub-account is frozen.",
            Self::SubAccountPermissionError => "Sub-account permission error.",
            Self::MasterAccountFrozen => "Master account is frozen.",
            Self::MasterSubRelationshipExists => "Master-sub relationship already exists.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = GeneralCode;

    #[test]
    fn test_from_code_i32() {
        assert_eq!(Sut::from_code(10001), Some(Sut::InvalidParameters));
        assert_eq!(
            Sut::from_code(100615),
            Some(Sut::MasterSubRelationshipExists)
        );
        assert_eq!(Sut::from_code(99999), None);
    }

    #[test]
    fn test_display() {
        let error = Sut::InvalidParameters;
        assert_eq!(error.to_string(), "10001 - Invalid parameter(s).");
        let error = Sut::MasterSubRelationshipExists;
        assert_eq!(
            error.to_string(),
            "100615 - Master-sub relationship already exists."
        );
    }
}
