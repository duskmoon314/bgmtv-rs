#![doc = include_str!("../README.md")]

pub mod client;
pub mod types;

pub mod prelude {
    pub use crate::client::Client;

    pub use crate::types::*;

    pub use crate::error::*;
}

pub mod error {
    use error_set::error_set;
    error_set! {
        DepsError = {
            Reqwest(reqwest::Error),
            HeaderValueToStr(reqwest::header::ToStrError),
            InvalidUrl(url::ParseError),
            Serialize(serde_json::Error)
        };
        SearchSubjectsError = {
            #[display("Cannot build request to search subjects: {0}")]
            Builder(crate::client::subjects::SearchSubjectsExecutorBuilderError)
        } || DepsError;
        GetSubjectsError = {
            #[display("Cannot build request to get subjects: {0}")]
            Builder(crate::client::subjects::GetSubjectsExecutorBuilderError)
        } || DepsError;
    }
}
