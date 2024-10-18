//! mod client: Client and its methods.
//!
//! 此模块包含了 [`Client`] 结构体、其相关方法的辅助结构体与实现。

use derive_builder::{Builder, UninitializedFieldError};

pub(crate) const DEFAULT_USER_AGENT: &str = concat!(
    "duskmoon/bgmtv/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")",
);

/// # Client, API Wrapper
///
/// [`Client`] 是对 API 的封装，提供了对主要 API 的访问方法。如果有 API 的访问尚未实现，可以调用 [`Client::client`] 方法获取内部的
/// [`reqwest::Client`] 对象，然后自行实现。
///
/// ## Usage
///
/// [`Client`] 的构建高度依赖于 [`ClientBuilder`]，你可以参考 [`ClientBuilder`] 的说明来了解有哪些配置项可以设置。
///
/// 一般情况下，你可以直接使用 [`Client::new`] 来创建一个默认的 [`Client`] 对象用于开发。不过对于生产环境，强烈建议类似下面的示例来创建一个
/// 具有对应的 user agent 和 token 的 [`Client`] 对象。
///
/// ## Example
///
/// ```
/// # use bgmtv::prelude::*;
/// let client = Client::builder()
///     .user_agent("xxx/yyy/1.0")
///     .token("auth_token")
///     .build()
///     .unwrap();
///
/// assert_eq!(client.base_url(), "https://api.bgm.tv");
/// assert_eq!(client.user_agent(), "xxx/yyy/1.0");
/// assert_eq!(client.token(), Some("auth_token"));
/// ```
#[derive(Debug, Builder)]
pub struct Client {
    /// Base URL of the API.
    ///
    /// 默认值为 "<https://api.bgm.tv>"。一般情况下不需要修改。
    #[builder(default = "https://api.bgm.tv".to_string())]
    pub(crate) base_url: String,

    /// User agent.
    ///
    /// 根据 API 要求，此项需要设置为 `<开发者>/<应用名>/<版本号>` 的格式，以便于 bgm.tv 识别。
    ///
    /// 本 crate 提供了一个默认值，即 `duskmoon/bgmtv/<version>`。不过强烈建议开发者自行设置。
    #[builder(default, setter(into, strip_option))]
    pub(crate) user_agent: Option<String>,

    /// Authorization token.
    ///
    /// 用于访问需要授权的 API。如果不需要授权，可以不设置。
    #[builder(default, setter(into, strip_option))]
    pub(crate) token: Option<String>,

    /// Internal reqwest client.
    ///
    /// 一般情况下不需要设置。如果需要自定义 [`reqwest::Client`]，可以使用此项。
    #[builder(default = "self.default_client()?")]
    pub(crate) client: reqwest::Client,
}

impl ClientBuilder {
    fn default_client(&self) -> Result<reqwest::Client, UninitializedFieldError> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = self.token.clone().flatten() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }
        Ok(reqwest::Client::builder()
            .user_agent(
                self.user_agent
                    .clone()
                    .flatten()
                    .unwrap_or(DEFAULT_USER_AGENT.to_string()),
            )
            .default_headers(headers)
            .build()
            .map_err(|_| UninitializedFieldError::new("client"))?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// # Basic methods for [`Client`].
impl Client {
    /// 创建一个默认的 [`Client`] 对象。
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("Failed to build default client. Please report this issue.")
    }

    /// 创建一个 [`ClientBuilder`] 对象，用于构建 [`Client`] 对象。
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Get the base URL of the API.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the internal reqwest client.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the user agent.
    pub fn user_agent(&self) -> &str {
        self.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT)
    }

    /// Get the token.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

pub mod subjects;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_build() {
        let client = Client::new();
        assert_eq!(client.base_url(), "https://api.bgm.tv");
        assert_eq!(client.user_agent(), DEFAULT_USER_AGENT);
        assert!(client.token().is_none());

        let client = Client::builder()
            .user_agent("test_user_agent")
            .token("test_token")
            .build()
            .unwrap();
        assert_eq!(client.base_url(), "https://api.bgm.tv");
        assert_eq!(client.user_agent(), "test_user_agent");
        assert_eq!(client.token(), Some("test_token"));
    }
}
