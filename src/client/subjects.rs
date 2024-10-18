use std::ops::Deref;

use derive_builder::Builder;

use super::Client;
use crate::{error::*, types::*};

/// # Subjects Resource (条目资源)
///
/// | API                                         | Description      | Methods                                                    |
/// | :------------------------------------------ | :--------------- | :--------------------------------------------------------- |
/// | `POST /v0/search/subjects`                  | 条目搜索         | [`search_subjects`](Client::search_subjects)               |
/// | `GET /v0/subjects`                          | 浏览条目         | [`get_subjects`](Client::get_subjects)                     |
/// | `GET /v0/subjects/{subject_id}`             | 获取条目         | [`get_subject`](Client::get_subject)                       |
/// | `GET /v0/subjects/{subject_id}/image`       | 获取条目图片     | [`get_subject_image`](Client::get_subject_image)           |
/// | `GET /v0/subjects/{subject_id}/persons`     | 获取条目相关人物 | [`get_subject_persons`](Client::get_subject_persons)       |
/// | `GET /v0/subjects/{subject_id}/characters`  | 获取条目相关角色 | [`get_subject_characters`](Client::get_subject_characters) |
/// | `GET /v0/subjects/{subject_id}/subjects`    | 获取条目相关条目 | [`get_subject_subjects`](Client::get_subject_subjects)     |
impl Client {
    /// # 条目搜索 `POST /v0/search/subjects`
    ///
    /// 返回一个 Builder 模式的 [`SearchSubjectsExecutorBuilder`], 用于构建请求参数并发送请求
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = Client::new();
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
    pub fn search_subjects(&self) -> SearchSubjectsExecutorBuilder {
        SearchSubjectsExecutor::builder(self)
    }

    /// # 浏览条目 `GET /v0/subjects`
    ///
    /// 返回一个 Builder 模式的 [`GetSubjectsExecutorBuilder`], 用于构建请求参数并发送请求
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = Client::new();
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
    pub fn get_subjects(&self) -> GetSubjectsExecutorBuilder {
        GetSubjectsExecutor::builder(self)
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
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let subject = client.get_subject(3559).await.expect("Failed to get subject");
    ///
    /// assert_eq!(subject.name, "とある魔術の禁書目録");
    /// # });
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
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let image: Vec<u8> = client.get_subject_image(3559, ImageType::Small).await.expect("Failed to get subject image");
    ///
    /// assert_eq!(image.len(), 12020);
    /// # });
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
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let persons = client.get_subject_persons(3559).await.expect("Failed to get subject persons");
    ///
    /// let person = persons.iter().find(|p| p.id == 3608);
    /// assert_eq!(person.map(|p| p.name.as_str()), Some("鎌池和馬"));
    /// # });
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
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let characters = client.get_subject_characters(3559).await.expect("Failed to get subject characters");
    ///
    /// let character = characters.iter().find(|c| c.id == 3498);
    /// assert_eq!(character.map(|c| c.name.as_str()), Some("上条当麻"));
    /// # });
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
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let subjects = client.get_subject_subjects(3559).await.expect("Failed to get subject subjects");
    ///
    /// let subject = subjects.iter().find(|s| s.id == 3582);
    /// assert_eq!(subject.map(|s| s.name.as_str()), Some("とある魔術の禁書目録外伝 とある科学の超電磁砲"));
    /// # });
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

#[derive(Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct SearchSubjectsExecutor<'a> {
    #[doc(hidden)]
    client: &'a Client,

    /// 关键词
    #[builder(setter(into))]
    keyword: String,

    /// 排序方式
    sort: SortType,

    /// 返回数量
    #[builder(default)]
    limit: Option<u64>,

    /// 偏移量
    #[builder(default)]
    offset: Option<u64>,

    /// 过滤条件
    filter: SearchSubjectsFilter,
}

impl Deref for SearchSubjectsExecutor<'_> {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        self.client
    }
}

impl SearchSubjectsExecutor<'_> {
    pub fn builder(client: &Client) -> SearchSubjectsExecutorBuilder {
        SearchSubjectsExecutorBuilder::default().client(client)
    }

    pub async fn send(&self) -> Result<SearchSubjects, SearchSubjectsError> {
        let url = format!("{}/v0/search/subjects", self.client.base_url);
        let mut url = url::Url::parse(&url)?;

        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(offset) = self.offset {
            url.query_pairs_mut()
                .append_pair("offset", &offset.to_string());
        }

        let req = self
            .client()
            .post(url)
            .json(&SearchSubjectsBody {
                keyword: self.keyword.clone(),
                sort: self.sort,
                filter: self.filter.clone(),
            })
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.client.execute(req).await?.error_for_status()?;

        let subjects: SearchSubjects = res.json().await?;

        Ok(subjects)
    }
}

impl SearchSubjectsExecutorBuilder<'_> {
    /// 发送请求
    ///
    /// 此方法会先调用 [`build`](SearchSubjectsExecutorBuilder::build) 方法构建，然后发送请求
    pub async fn send(self) -> Result<SearchSubjects, SearchSubjectsError> {
        self.build()?.send().await
    }
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct GetSubjectsExecutor<'a> {
    #[doc(hidden)]
    client: &'a Client,

    /// 条目类型
    ///
    /// 参见 [`SubjectType`](crate::types::SubjectType)
    r#type: SubjectType,

    /// 条目分类
    ///
    /// 参见 [`SubjectCategory`](crate::types::SubjectCategory)
    #[builder(default)]
    cat: Option<SubjectCategory>,

    /// 是否为系列，仅对书籍类型条目有效
    #[builder(default)]
    series: Option<bool>,

    /// 平台，仅对游戏类型条目有效
    #[builder(default, setter(into))]
    platform: Option<String>,

    /// 排序方式，可选值为 `date`, `rank`
    #[builder(default, setter(into))]
    sort: Option<String>,

    /// 年份
    #[builder(default)]
    year: Option<u64>,

    /// 月份
    #[builder(default)]
    month: Option<u64>,

    /// 分页参数，返回数量
    #[builder(default)]
    limit: Option<u64>,

    /// 分页参数，偏移量
    #[builder(default)]
    offset: Option<u64>,
}

impl Deref for GetSubjectsExecutor<'_> {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        self.client
    }
}

impl<'a> GetSubjectsExecutor<'a> {
    fn builder(client: &'a Client) -> GetSubjectsExecutorBuilder {
        GetSubjectsExecutorBuilder::default().client(client)
    }

    pub async fn send(&self) -> Result<PagedSubject, GetSubjectsError> {
        let url = format!("{}/v0/subjects", self.client.base_url);
        let mut url = url::Url::parse(&url)?;

        url.query_pairs_mut()
            .append_pair("type", &serde_json::to_string(&self.r#type)?);

        if let Some(cat) = &self.cat {
            url.query_pairs_mut()
                .append_pair("cat", &serde_json::to_string(cat)?);
        }

        if let Some(series) = self.series {
            url.query_pairs_mut()
                .append_pair("series", &series.to_string());
        }

        if let Some(platform) = &self.platform {
            url.query_pairs_mut().append_pair("platform", platform);
        }

        if let Some(sort) = &self.sort {
            url.query_pairs_mut().append_pair("sort", sort);
        }

        if let Some(year) = self.year {
            url.query_pairs_mut().append_pair("year", &year.to_string());
        }

        if let Some(month) = self.month {
            url.query_pairs_mut()
                .append_pair("month", &month.to_string());
        }

        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(offset) = self.offset {
            url.query_pairs_mut()
                .append_pair("offset", &offset.to_string());
        }

        let req = self
            .client()
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.client.execute(req).await?.error_for_status()?;

        let subjects: PagedSubject = res.json().await?;

        Ok(subjects)
    }
}

impl GetSubjectsExecutorBuilder<'_> {
    /// 发送请求
    ///
    /// 此方法会先调用 [`build`](GetSubjectsExecutorBuilder::build) 方法构建，然后发送请求
    pub async fn send(self) -> Result<PagedSubject, GetSubjectsError> {
        self.build()?.send().await
    }
}
