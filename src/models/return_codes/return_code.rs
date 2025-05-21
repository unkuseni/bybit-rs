use crate::prelude::*;
use serde::de::{self, Unexpected, Visitor};
use std::fmt;

/// Enum representing Bybit API V5 return/error codes.
///
/// See: https://bybit-exchange.github.io/docs/v5/error
#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, derive_more::Debug, derive_more::From)]
#[repr(i32)]
#[display("{}({}) - {}", self.discriminator(), self.code(), self.message())]
pub enum ReturnCode {
    /// Asset return/error codes.
    Asset(AssetCode),

    /// Earn return/error codes.
    Earn(EarnCode),

    /// Crypto Loan return/error codes.
    CryptoLoan(CryptoLoanCode),

    /// Spot Leverage Token return/error codes.
    SpotLeverageToken(SpotLeverageTokenCode),

    /// Spot Trade return/error codes.
    SpotTrade(SpotTradeCode),

    /// Spot Margin return/error codes.
    SpotMargin(SpotMarginCode),

    /// UTA return/error codes.
    Uta(UtaCode),

    /// General return/error codes.
    General(GeneralCode),
}

impl ReturnCode {
    pub fn from_code<T>(code: T) -> Option<Self>
    where
        T: Into<i32> + Copy,
    {
        if let Some(code) = AssetCode::from_code(code) {
            return Some(Self::Asset(code));
        }
        if let Some(code) = EarnCode::from_code(code) {
            return Some(Self::Earn(code));
        }
        if let Some(code) = CryptoLoanCode::from_code(code) {
            return Some(Self::CryptoLoan(code));
        }
        if let Some(code) = SpotLeverageTokenCode::from_code(code) {
            return Some(Self::SpotLeverageToken(code));
        }
        if let Some(code) = SpotTradeCode::from_code(code) {
            return Some(Self::SpotTrade(code));
        }
        if let Some(code) = SpotMarginCode::from_code(code) {
            return Some(Self::SpotMargin(code));
        }
        if let Some(code) = UtaCode::from_code(code) {
            return Some(Self::Uta(code));
        }
        if let Some(code) = GeneralCode::from_code(code) {
            return Some(Self::General(code));
        }
        None
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::Asset(code) => code.message(),
            Self::Earn(code) => code.message(),
            Self::CryptoLoan(code) => code.message(),
            Self::SpotLeverageToken(code) => code.message(),
            Self::SpotTrade(code) => code.message(),
            Self::SpotMargin(code) => code.message(),
            Self::Uta(code) => code.message(),
            Self::General(code) => code.message(),
        }
    }

    pub fn discriminator(&self) -> &'static str {
        match self {
            Self::Asset(_) => "Asset",
            Self::Earn(_) => "Earn",
            Self::CryptoLoan(_) => "CryptoLoan",
            Self::SpotLeverageToken(_) => "SpotLeverageToken",
            Self::SpotTrade(_) => "SpotTrade",
            Self::SpotMargin(_) => "SpotMargin",
            Self::Uta(_) => "Uta",
            Self::General(_) => "General",
        }
    }

    pub fn code(&self) -> i32 {
        match self {
            Self::Asset(code) => *code as i32,
            Self::Earn(code) => *code as i32,
            Self::CryptoLoan(code) => *code as i32,
            Self::SpotLeverageToken(code) => *code as i32,
            Self::SpotTrade(code) => *code as i32,
            Self::SpotMargin(code) => *code as i32,
            Self::Uta(code) => *code as i32,
            Self::General(code) => *code as i32,
        }
    }
}

impl Serialize for ReturnCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.code())
    }
}

impl<'de> Deserialize<'de> for ReturnCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReturnCodeVisitor;

        impl<'de> Visitor<'de> for ReturnCodeVisitor {
            type Value = ReturnCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid Bybit return code as i32")
            }

            fn visit_i32<E>(self, value: i32) -> Result<ReturnCode, E>
            where
                E: de::Error,
            {
                ReturnCode::from_code(value).ok_or_else(|| {
                    de::Error::invalid_value(Unexpected::Signed(value as i64), &self)
                })
            }

            fn visit_u32<E>(self, value: u32) -> Result<ReturnCode, E>
            where
                E: de::Error,
            {
                let value_i32 = value as i32;
                ReturnCode::from_code(value_i32).ok_or_else(|| {
                    de::Error::invalid_value(Unexpected::Unsigned(value as u64), &self)
                })
            }
        }

        deserializer.deserialize_i32(ReturnCodeVisitor)
    }
}
