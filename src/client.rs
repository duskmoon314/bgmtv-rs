use derive_builder::{Builder, UninitializedFieldError};
// use typed_builder::TypedBuilder;

pub mod subjects;

pub(crate) const DEFAULT_USER_AGENT: &str = concat!(
    "duskmoon/bgmtv/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ")",
);

/// # Client, API Wrapper
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
    #[builder(default = "https://api.bgm.tv".to_string())]
    pub(crate) base_url: String,

    #[builder(default, setter(into, strip_option))]
    pub(crate) user_agent: Option<String>,

    #[builder(default, setter(into, strip_option))]
    pub(crate) token: Option<String>,

    // #[builder(default = {
    //     let mut headers = reqwest::header::HeaderMap::new();
    //     if let Some(token) = token.as_ref() {
    //         headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
    //     }
    //     reqwest::Client::builder()
    //         .user_agent(user_agent.as_ref().unwrap_or(&DEFAULT_USER_AGENT.to_string()))
    //         .default_headers(headers)
    //         .build().unwrap()
    // })]
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

impl Client {
    /// Create a new default client.
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("Failed to build default client. Please report this issue.")
    }

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
