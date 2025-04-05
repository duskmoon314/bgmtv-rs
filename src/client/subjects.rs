//! # Subjects Resource (条目资源)

use std::ops::Deref;

use derive_builder::Builder;

use super::Client;
use crate::{error::*, types::*};

/// # 条目搜索执行器
///
/// 此结构用于构建请求参数并发送请求
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
    /// 返回一个 Builder 模式的 [`SearchSubjectsExecutorBuilder`], 用于构建请求参数并发送请求
    pub(super) fn builder(client: &Client) -> SearchSubjectsExecutorBuilder {
        SearchSubjectsExecutorBuilder::default().client(client)
    }

    /// 发送请求
    ///
    /// 根据构建的请求参数发送请求，并返回搜索结果
    pub async fn send(&self) -> Result<SearchSubjects, SearchSubjectsError> {
        let url = format!("{}/v0/search/subjects", self.client.base_url);

        let req = self
            .client()
            .post(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&[("limit", &self.limit)])
            .query(&[("offset", &self.offset)])
            .json(&SearchSubjectsBody {
                keyword: self.keyword.clone(),
                sort: self.sort,
                filter: self.filter.clone(),
            })
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

/// # 浏览条目执行器
///
/// 此结构用于构建请求参数并发送请求
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
    /// 返回一个 Builder 模式的 [`GetSubjectsExecutorBuilder`], 用于构建请求参数并发送请求
    pub(super) fn builder(client: &'a Client) -> GetSubjectsExecutorBuilder<'a> {
        GetSubjectsExecutorBuilder::default().client(client)
    }

    /// 发送请求
    ///
    /// 根据构建的请求参数发送请求，并返回搜索结果
    pub async fn send(&self) -> Result<PagedSubject, GetSubjectsError> {
        let url = format!("{}/v0/subjects", self.client.base_url);

        let req = self
            .client()
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&[("type", &self.r#type)])
            .query(&[("cat", &self.cat)])
            .query(&[("series", &self.series)])
            .query(&[("platform", &self.platform)])
            .query(&[("sort", &self.sort)])
            .query(&[("year", &self.year)])
            .query(&[("month", &self.month)])
            .query(&[("limit", &self.limit)])
            .query(&[("offset", &self.offset)])
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
