use super::Client;
use crate::types::*;

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
    /// ## Arguments
    ///
    /// * `keyword` - 关键词
    /// * `sort` - 排序方式
    /// * `limit` - 返回数量
    /// * `offset` - 偏移量
    /// * `filter` - 过滤条件
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let filter = SearchSubjectsFilter {
    ///     r#type: vec![SubjectType::Anime],
    ///     ..Default::default()
    /// };
    /// let subjects = client.search_subjects("魔法禁书目录", SortType::Match, Some(1), Some(0), filter).await.expect("Failed to search subjects");
    ///
    /// assert_eq!(subjects.data[0].id, 1014);
    /// assert_eq!(subjects.data[0].name, "とある魔術の禁書目録");
    /// # });
    /// ```
    pub async fn search_subjects(
        &self,
        keyword: impl Into<String>,
        sort: SortType,
        limit: Option<u64>,
        offset: Option<u64>,
        filter: SearchSubjectsFilter,
    ) -> Result<SearchSubjects, Error> {
        let url = format!("{}/v0/search/subjects", self.base_url);
        let mut url = url::Url::parse(&url)?;

        if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(offset) = offset {
            url.query_pairs_mut()
                .append_pair("offset", &offset.to_string());
        }

        let req = self
            .client
            .post(url)
            .json(&SearchSubjectsBody {
                keyword: keyword.into(),
                sort,
                filter,
            })
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: SearchSubjects = res.json().await?;

        Ok(subjects)
    }

    /// # 浏览条目 `GET /v0/subjects`
    ///
    /// ## Arguments
    ///
    /// * `type` - 条目类型
    /// * `cat` - 条目分类
    /// * `series` - 是否是系列
    /// * `platform` - 平台
    /// * `sort` - 排序方式
    /// * `year` - 年份
    /// * `month` - 月份
    /// * `limit` - 分页参数
    /// * `offset` - 分页参数
    ///
    /// ## Example
    ///
    /// ```
    /// # use bgmtv::prelude::*;
    /// # tokio_test::block_on(async {
    /// # let client = Client::new();
    /// let subjects = client.get_subjects(
    ///     SubjectType::Book,
    ///     Some(SubjectCategory::Book(SubjectBookCategory::Novel)),
    ///     None,
    ///     None,
    ///     Some("date"),
    ///     Some(2023),
    ///     None,
    ///     Some(1),
    ///     Some(0),
    /// ).await.expect("Failed to get subjects");
    ///
    /// assert_eq!(subjects.data[0].id, 469252);
    /// });
    /// ```
    pub async fn get_subjects(
        &self,
        r#type: SubjectType,
        cat: Option<SubjectCategory>,
        series: Option<bool>,
        platform: Option<String>,
        sort: Option<impl Into<String>>,
        year: Option<u64>,
        month: Option<u64>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<PagedSubject, Error> {
        let url = format!("{}/v0/subjects", self.base_url);
        let mut url = url::Url::parse(&url)?;

        url.query_pairs_mut()
            .append_pair("type", &serde_json::to_string(&r#type)?);

        if let Some(cat) = cat {
            url.query_pairs_mut()
                .append_pair("cat", &serde_json::to_string(&cat)?);
        }

        if let Some(series) = series {
            url.query_pairs_mut()
                .append_pair("series", &series.to_string());
        }

        if let Some(platform) = platform {
            url.query_pairs_mut().append_pair("platform", &platform);
        }

        if let Some(sort) = sort {
            url.query_pairs_mut().append_pair("sort", &sort.into());
        }

        if let Some(year) = year {
            url.query_pairs_mut().append_pair("year", &year.to_string());
        }

        if let Some(month) = month {
            url.query_pairs_mut()
                .append_pair("month", &month.to_string());
        }

        if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        if let Some(offset) = offset {
            url.query_pairs_mut()
                .append_pair("offset", &offset.to_string());
        }

        let req = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .build()?;

        let res = self.client.execute(req).await?.error_for_status()?;

        let subjects: PagedSubject = res.json().await?;

        Ok(subjects)
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
    pub async fn get_subject(&self, subject_id: u64) -> Result<Subject, Error> {
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
    ) -> Result<Vec<u8>, Error> {
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
    pub async fn get_subject_persons(&self, subject_id: u64) -> Result<Vec<RelatedPerson>, Error> {
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
    ) -> Result<Vec<RelatedCharacter>, Error> {
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
    ) -> Result<Vec<SubjectRelation>, Error> {
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
