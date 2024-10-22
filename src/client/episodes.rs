//! # Episodes Resource (章节资源)

use std::ops::Deref;

use derive_builder::Builder;

use super::Client;
use crate::{error::*, types::*};

/// # 获取章节列表执行器
///
/// 此结构用于构建请求参数并发送请求
#[derive(Debug, Builder)]
#[builder(pattern = "owned", setter(strip_option))]
pub struct GetEpisodesExecutor<'a> {
    #[doc(hidden)]
    client: &'a Client,

    /// 作品 ID
    subject_id: u64,

    /// 章节类型
    #[builder(default)]
    r#type: Option<EpisodeType>,

    /// 返回数量
    #[builder(default)]
    limit: Option<u64>,

    /// 偏移量
    #[builder(default)]
    offset: Option<u64>,
}

impl Deref for GetEpisodesExecutor<'_> {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        self.client
    }
}

impl GetEpisodesExecutor<'_> {
    /// 返回一个 Builder 模式的 [`GetEpisodesExecutorBuilder`], 用于构建请求参数
    pub(super) fn builder(client: &Client, subject_id: u64) -> GetEpisodesExecutorBuilder {
        GetEpisodesExecutorBuilder::default()
            .subject_id(subject_id)
            .client(client)
    }

    /// 发送请求
    ///
    /// 根据构建的请求参数发送请求，并返回搜索结果
    pub async fn send(&self) -> Result<PagedEpisode, GetEpisodesError> {
        let url = format!("{}/v0/episodes", self.base_url());

        let req = self
            .client()
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .query(&[("subject_id", self.subject_id)])
            .query(&[("type", self.r#type)])
            .query(&[("limit", self.limit)])
            .query(&[("offset", self.offset)])
            .build()?;

        let resp = self.client().execute(req).await?;

        let episodes: PagedEpisode = resp.json().await?;

        Ok(episodes)
    }
}

impl GetEpisodesExecutorBuilder<'_> {
    /// 发送请求
    ///
    /// 此方法会先调用 [`build`](GetEpisodesExecutorBuilder::build) 方法构建请求参数，然后发送请求
    pub async fn send(self) -> Result<PagedEpisode, GetEpisodesError> {
        self.build()?.send().await
    }
}
