#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod types;

/// Prelude module
///
/// 此 mod 提供了本 crate 中所有 API 的预导入项，使用 `pub use` 导入。
pub mod prelude {
    pub use crate::client::Client;

    pub use crate::types::*;

    pub use crate::error::*;
}

/// Error types
///
/// 此 mod 提供了本 crate 中所有 API 返回的错误类型，使用 `error_set!` 宏定义。
pub mod error {
    use error_set::error_set;
    error_set! {
        /// Error from dependencies
        DepsError = {
            /// Error from reqwest
            ///
            /// 这是 [`reqwest`] 提供的基础错误类型，几乎大部分 API 调用都可能返回这个错误。
            Reqwest(reqwest::Error),
            /// Error of converting header value to string
            ///
            /// 这是 [`reqwest::header::HeaderValue`] 转换为字符串时可能返回的错误。
            HeaderValueToStr(reqwest::header::ToStrError),
            /// Error of parsing URL
            ///
            /// 这是 [`url::ParseError`] 在解析 URL 时可能返回的错误。
            InvalidUrl(url::ParseError),
            /// Error of serializing to JSON
            ///
            /// 这会出现在将某些类型序列化为 JSON 时，目前是用于将一些 enum 转换为对应的 JSON 字符串。
            Serialize(serde_json::Error)
        };

        /// Error for [Client::search_subjects](crate::client::Client::search_subjects)
        SearchSubjectsError = {
            /// Error of building [SearchSubjectsExecutor](crate::client::subjects::SearchSubjectsExecutor)
            #[display("Cannot build request to search subjects: {0}")]
            Builder(crate::client::subjects::SearchSubjectsExecutorBuilderError)
        } || DepsError;

        /// Error for [Client::get_subjects](crate::client::Client::get_subjects)
        GetSubjectsError = {
            /// Error of building [GetSubjectsExecutor](crate::client::subjects::GetSubjectsExecutor)
            #[display("Cannot build request to get subjects: {0}")]
            Builder(crate::client::subjects::GetSubjectsExecutorBuilderError)
        } || DepsError;

        /// Error for [Client::get_episodes](crate::client::Client::get_episodes)
        GetEpisodesError = {
            /// Error of building [GetEpisodesExecutor](crate::client::episodes::GetEpisodesExecutor)
            #[display("Cannot build request to get episodes: {0}")]
            Builder(crate::client::episodes::GetEpisodesExecutorBuilderError)
        } || DepsError;
    }
}
