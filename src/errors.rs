use serde::Deserialize;
use error_chain::error_chain;

#[derive(Debug, Deserialize)]
pub struct BybitContentError {
    pub code: i16,
    pub msg: String,
}

error_chain! {
    errors {
        BybitError(response: BybitContentError)

        KlineValueMissingError(index: usize, name: &'static str) {
            description("invalid Vec for Kline"),
            display("{} at {} is missing", name, index),
        }
     }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        Tungstenite(tokio_tungstenite::tungstenite::Error);
        TimestampError(std::time::SystemTimeError);
        SerdeError(serde::de::value::Error);
    }
}