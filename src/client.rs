//! mod client: Client and its methods.
//!
//! 此模块包含了 [`Client`] 结构体、其相关方法的辅助结构体与实现。

use derive_builder::{Builder, UninitializedFieldError};

use crate::prelude::*;

pub mod episodes;
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
        let client = reqwest::Client::builder()
            // .use_rustls_tls()
            .user_agent(
                self.user_agent
                    .clone()
                    .flatten()
                    .unwrap_or(DEFAULT_USER_AGENT.to_string()),
            )
            .default_headers(headers)
            .build()
            .map_err(|_| UninitializedFieldError::new("client"))?;

        Ok(client)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// # Basic methods for [`Client`]
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

/// # Subjects Resource (条目资源)
///
/// | API                                         | Description      | Methods                                                    |
/// | :------------------------------------------ | :--------------- | :--------------------------------------------------------- |
/// | `POST /v0/search/subjects`                  | 条目搜索         | [`search_subjects`](Client::search_subjects)               |
/// | `GET  /v0/subjects`                         | 浏览条目         | [`get_subjects`](Client::get_subjects)                     |
/// | `GET  /v0/subjects/{subject_id}`            | 获取条目         | [`get_subject`](Client::get_subject)                       |
/// | `GET  /v0/subjects/{subject_id}/image`      | 获取条目图片     | [`get_subject_image`](Client::get_subject_image)           |
/// | `GET  /v0/subjects/{subject_id}/persons`    | 获取条目相关人物 | [`get_subject_persons`](Client::get_subject_persons)       |
/// | `GET  /v0/subjects/{subject_id}/characters` | 获取条目相关角色 | [`get_subject_characters`](Client::get_subject_characters) |
/// | `GET  /v0/subjects/{subject_id}/subjects`   | 获取条目相关条目 | [`get_subject_subjects`](Client::get_subject_subjects)     |
impl Client {
    /// # 条目搜索 `POST /v0/search/subjects`
    ///
    /// 返回一个 Builder 模式的 [`SearchSubjectsExecutorBuilder`](subjects::SearchSubjectsExecutorBuilder), 用于构建请求参数并发送请求
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let subjects = client.search_subjects()
    ///     .keyword("魔法禁书目录")
    ///     .sort(SortType::Match)
    ///     .limit(1)
    ///     .offset(0)
    ///     .filter(
    ///         SearchSubjectsFilter::builder()
    ///         .r#type(SubjectType::Anime)
    ///         .build()?
    ///     )
    ///     .send()
    ///     .await?;
    ///
    /// assert_eq!(subjects.data[0].id, 1014);
    /// assert_eq!(subjects.data[0].name, "とある魔術の禁書目録");
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_subjects(&self) -> subjects::SearchSubjectsExecutorBuilder {
        subjects::SearchSubjectsExecutor::builder(self)
    }

    /// # 浏览条目 `GET /v0/subjects`
    ///
    /// 返回一个 Builder 模式的 [`GetSubjectsExecutorBuilder`](subjects::GetSubjectsExecutorBuilder), 用于构建请求参数并发送请求
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let subjects = client.get_subjects()
    ///     .r#type(SubjectType::Book)
    ///     .cat(SubjectCategory::Book(SubjectBookCategory::Novel))
    ///     .sort("date")
    ///     .year(2023)
    ///     .limit(1)
    ///     .send()
    ///     .await?;
    ///
    /// assert_eq!(subjects.data[0].id, 469252);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_subjects(&self) -> subjects::GetSubjectsExecutorBuilder {
        subjects::GetSubjectsExecutor::builder(self)
    }

    /// # 获取条目 `GET /v0/subjects/{subject_id}`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let subject = client.get_subject(3559).await?;
    ///
    /// assert_eq!(subject.name, "とある魔術の禁書目録");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_subject(&self, subject_id: u64) -> Result<Subject, DepsError> {
        let url = format!("{}/v0/subjects/{}", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subject: Subject = res.json().await?;

        Ok(subject)
    }

    /// # 获取条目图片 `GET /v0/subjects/{subject_id}/image`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    /// * `type` - 图片类型
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let image: Vec<u8> = client.get_subject_image(3559, ImageType::Small).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_subject_image(
        &self,
        subject_id: u64,
        image_type: ImageType,
    ) -> Result<Vec<u8>, DepsError> {
        let url = format!("{}/v0/subjects/{}/image", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .query(&[("type", image_type)])
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let image = res.bytes().await?;

        Ok(image.to_vec())
    }

    /// # 获取条目相关人物 `GET /v0/subjects/{subject_id}/persons`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let persons = client.get_subject_persons(3559).await?;
    ///
    /// let person = persons.iter().find(|p| p.id == 3608);
    /// assert_eq!(person.map(|p| p.name.as_str()), Some("鎌池和馬"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_subject_persons(
        &self,
        subject_id: u64,
    ) -> Result<Vec<RelatedPerson>, DepsError> {
        let url = format!("{}/v0/subjects/{}/persons", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let persons: Vec<RelatedPerson> = res.json().await?;

        Ok(persons)
    }

    /// # 获取条目相关角色 `GET /v0/subjects/{subject_id}/characters`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let characters = client.get_subject_characters(3559).await.expect("Failed to get subject characters");
    ///
    /// let character = characters.iter().find(|c| c.id == 3498);
    /// assert_eq!(character.map(|c| c.name.as_str()), Some("上条当麻"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_subject_characters(
        &self,
        subject_id: u64,
    ) -> Result<Vec<RelatedCharacter>, DepsError> {
        let url = format!("{}/v0/subjects/{}/characters", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let characters: Vec<RelatedCharacter> = res.json().await?;

        Ok(characters)
    }

    /// # 获取条目相关条目 `GET /v0/subjects/{subject_id}/subjects`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let subjects = client.get_subject_subjects(3559).await?;
    ///
    /// let subject = subjects.iter().find(|s| s.id == 3582);
    /// assert_eq!(subject.map(|s| s.name.as_str()), Some("とある魔術の禁書目録外伝 とある科学の超電磁砲"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_subject_subjects(
        &self,
        subject_id: u64,
    ) -> Result<Vec<SubjectRelation>, DepsError> {
        let url = format!("{}/v0/subjects/{}/subjects", self.base_url, subject_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: Vec<SubjectRelation> = res.json().await?;

        Ok(subjects)
    }
}

/// # Episodes Resource (章节资源)
///
/// | API                             | Description  | Methods                                |
/// | :------------------------------ | :----------- | :------------------------------------- |
/// | `GET /v0/episodes`              | 获取章节列表 | [`get_episodes`](Client::get_episodes) |
/// | `GET /v0/episodes/{episode_id}` | 获取章节信息 | [`get_episode`](Client::get_episode)   |
impl Client {
    /// # 获取章节列表 `GET /v0/episodes`
    ///
    /// ## Arguments
    ///
    /// * `subject_id` - 条目 ID
    ///
    /// ## Returns
    ///
    /// 返回一个 Builder 模式的 [`GetEpisodesExecutorBuilder`](episodes::GetEpisodesExecutorBuilder), 用于构建请求参数并发送请求
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let episodes = client.get_episodes(1014)
    ///     .r#type(EpisodeType::MainStory)
    ///     .limit(1)
    ///     .send()
    ///     .await?;
    ///
    /// assert_eq!(episodes.data[0].id, 1731);
    /// assert_eq!(episodes.data[0].name, "学園都市");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_episodes(&self, subject_id: u64) -> episodes::GetEpisodesExecutorBuilder {
        episodes::GetEpisodesExecutor::builder(self, subject_id)
    }

    /// # 获取章节信息 `GET /v0/episodes/{episode_id}`
    ///
    /// ## Arguments
    ///
    /// * `episode_id` - 章节 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let episode = client.get_episode(1731).await?;
    ///
    /// assert_eq!(episode.name, "学園都市");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_episode(&self, episode_id: u64) -> Result<Episode, DepsError> {
        let url = format!("{}/v0/episodes/{}", self.base_url, episode_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let episode: Episode = res.json().await?;

        Ok(episode)
    }
}

/// # Characters Resource (角色资源)
///
/// | API                                           | Description      | Methods                                                    |
/// | :-------------------------------------------- | :--------------- | :--------------------------------------------------------- |
/// | `GET  /v0/characters/{character_id}`          | 获取角色信息     | [`get_character`](Client::get_character)                   |
/// | `GET  /v0/characters/{character_id}/image`    | 获取角色图片     | [`get_character_image`](Client::get_character_image)       |
/// | `GET  /v0/characters/{character_id}/subjects` | 获取角色相关条目 | [`get_character_subjects`](Client::get_character_subjects) |
/// | `GET  /v0/characters/{character_id}/persons`  | 获取角色相关人物 | [`get_character_persons`](Client::get_character_persons)   |
impl Client {
    /// # 获取角色信息 `GET /v0/characters/{character_id}`
    ///
    /// ## Arguments
    ///
    /// * `character_id` - 角色 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let character = client.get_character(3498).await?;
    ///
    /// assert_eq!(character.name, "上条当麻");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character(&self, character_id: u64) -> Result<CharacterDetail, DepsError> {
        let url = format!("{}/v0/characters/{}", self.base_url, character_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let character: CharacterDetail = res.json().await?;

        Ok(character)
    }

    /// # 获取角色图片 `GET /v0/characters/{character_id}/image`
    ///
    /// ## Arguments
    ///
    /// * `character_id` - 角色 ID
    /// * `image_type` - 图片类型, 支持 `Small`, `Grid`, `Medium`, `Large`
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let image = client.get_character_image(3498, ImageType::Small).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character_image(
        &self,
        character_id: u64,
        image_type: ImageType,
    ) -> Result<Vec<u8>, DepsError> {
        let url = format!("{}/v0/characters/{}/image", self.base_url, character_id);

        let req = self
            .client
            .get(url)
            .query(&[("type", image_type)])
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let image = res.bytes().await?;

        Ok(image.to_vec())
    }

    /// # 获取角色相关条目 `GET /v0/characters/{character_id}/subjects`
    ///
    /// ## Arguments
    ///
    /// * `character_id` - 角色 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let subjects = client.get_character_subjects(3498).await?;
    ///
    /// let subject = subjects.iter().find(|s| s.id == 3559);
    /// assert_eq!(subject.map(|s| s.name.as_str()), Some("とある魔術の禁書目録"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character_subjects(
        &self,
        character_id: u64,
    ) -> Result<Vec<RelatedSubject>, DepsError> {
        let url = format!("{}/v0/characters/{}/subjects", self.base_url, character_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: Vec<RelatedSubject> = res.json().await?;

        Ok(subjects)
    }

    /// # 获取角色相关人物 `GET /v0/characters/{character_id}/persons`
    ///
    /// ## Arguments
    ///
    /// * `character_id` - 角色 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client= Client::new();
    /// let persons = client.get_character_persons(3498).await?;
    ///
    /// let person = persons.iter().find(|p| p.subject_id == 1014);
    /// assert_eq!(person.map(|p| p.name.as_str()), Some("阿部敦"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character_persons(
        &self,
        character_id: u64,
    ) -> Result<Vec<CharacterPerson>, DepsError> {
        let url = format!("{}/v0/characters/{}/persons", self.base_url, character_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let persons: Vec<CharacterPerson> = res.json().await?;

        Ok(persons)
    }
}

/// # Persons Resource (人物资源)
///
/// | API                                       | Description      | Methods                                                  |
/// | :---------------------------------------- | :--------------- | :------------------------------------------------------- |
/// | `GET  /v0/persons/{person_id}`            | 获取人物信息     | [`get_person`](Client::get_person)                       |
/// | `GET  /v0/persons/{person_id}/image`      | 获取人物图片     | [`get_person_image`](Client::get_person_image)           |
/// | `GET  /v0/persons/{person_id}/subjects`   | 获取人物相关条目 | [`get_person_subjects`](Client::get_person_subjects)     |
/// | `GET  /v0/persons/{person_id}/characters` | 获取人物相关角色 | [`get_person_characters`](Client::get_person_characters) |
impl Client {
    /// # 获取人物信息 `GET /v0/persons/{person_id}`
    ///
    /// ## Arguments
    ///
    /// * `person_id` - 人物 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client= Client::new();
    /// let person = client.get_person(3608).await?;
    ///
    /// assert_eq!(person.name, "鎌池和馬");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person(&self, person_id: u64) -> Result<PersonDetail, DepsError> {
        let url = format!("{}/v0/persons/{}", self.base_url, person_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let person: PersonDetail = res.json().await?;

        Ok(person)
    }

    /// # 获取人物图片 `GET /v0/persons/{person_id}/image`
    ///
    /// ## Arguments
    ///
    /// * `person_id` - 人物 ID
    /// * `type` - 图片类型, 支持 `Small`, `Grid`, `Medium`, `Large`
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client= Client::new();
    /// let image: Vec<u8> = client.get_person_image(3608, ImageType::Small).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person_image(
        &self,
        person_id: u64,
        image_type: ImageType,
    ) -> Result<Vec<u8>, DepsError> {
        let url = format!("{}/v0/persons/{}/image", self.base_url, person_id);

        let req = self
            .client
            .get(url)
            .query(&[("type", image_type)])
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let image = res.bytes().await?;

        Ok(image.to_vec())
    }

    /// # 获取人物相关条目 `GET /v0/persons/{person_id}/subjects`
    ///
    /// ## Arguments
    ///
    /// * `person_id` - 人物 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client= Client::new();
    /// let subjects = client.get_person_subjects(3608).await?;
    ///
    /// let subject = subjects.iter().find(|s| s.id == 1014);
    /// assert_eq!(subject.map(|s| s.name.as_str()), Some("とある魔術の禁書目録"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person_subjects(
        &self,
        person_id: u64,
    ) -> Result<Vec<RelatedSubject>, DepsError> {
        let url = format!("{}/v0/persons/{}/subjects", self.base_url, person_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: Vec<RelatedSubject> = res.json().await?;

        Ok(subjects)
    }

    /// # 获取人物相关角色 `GET /v0/persons/{person_id}/characters`
    ///
    /// ## Arguments
    ///
    /// * `person_id` - 人物 ID
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client= Client::new();
    /// let characters = client.get_person_characters(5015).await?;
    ///
    /// let character = characters.iter().find(|c| c.id == 3498);
    /// assert_eq!(character.map(|c| c.name.as_str()), Some("上条当麻"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person_characters(
        &self,
        person_id: u64,
    ) -> Result<Vec<PersonCharacter>, DepsError> {
        let url = format!("{}/v0/persons/{}/characters", self.base_url, person_id);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let characters: Vec<PersonCharacter> = res.json().await?;

        Ok(characters)
    }
}

/// # User Resource (用户资源)
///
/// | API                               | Description  | Methods                                      |
/// | :-------------------------------- | :----------- | :------------------------------------------- |
/// | `GET /v0/users/{username}`        | 获取用户信息 | [`get_user`](Client::get_user)               |
/// | `GET /v0/users/{username}/avatar` | 获取用户头像 | [`get_user_avatar`](Client::get_user_avatar) |
/// | `GET /v0/me`                      | 获取当前用户 | [`get_me`](Client::get_me)                   |
impl Client {
    /// # 获取用户信息 `GET /v0/users/{username}`
    ///
    /// ## Arguments
    ///
    /// * `username` - 用户名
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let user = client.get_user("sai").await?;
    ///
    /// assert_eq!(user.username, "sai");
    /// assert_eq!(user.id, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self, username: &str) -> Result<User, DepsError> {
        let url = format!("{}/v0/users/{}", self.base_url, username);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let user: User = res.json().await?;

        Ok(user)
    }

    /// # 获取用户头像 `GET /v0/users/{username}/avatar`
    ///
    /// ## Arguments
    ///
    /// * `username` - 用户名
    /// * `type` - 图片类型, 支持 `Small`, `Medium`, `Large`
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = Client::new();
    /// let image = client.get_user_avatar("sai", ImageType::Small).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_avatar(
        &self,
        username: &str,
        image_type: ImageType,
    ) -> Result<Vec<u8>, DepsError> {
        let url = format!("{}/v0/users/{}/avatar", self.base_url, username);

        let req = self
            .client
            .get(url)
            .query(&[("type", image_type)])
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let image = res.bytes().await?;

        Ok(image.to_vec())
    }

    /// # 获取当前用户 `GET /v0/me`
    ///
    /// <div class="warning">
    ///
    /// 此方法需要提供 token，你可以在 <https://next.bgm.tv/demo/access-token> 生成。
    ///
    /// </div>
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let token = std::env::var("BGMTV_TOKEN").expect("Please set BGMTV_TOKEN to test get_me");
    /// let client = Client::builder()
    ///     .token(token)
    ///     .build()?;
    /// let user = client.get_me().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_me(&self) -> Result<User, DepsError> {
        let url = format!("{}/v0/me", self.base_url);

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let user: User = res.json().await?;

        Ok(user)
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
