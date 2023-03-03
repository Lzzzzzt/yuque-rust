use std::borrow::Cow;

use chrono::{DateTime, Local};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    gen_random_slug, judge_status_code, time_serde, toc_serde, Toc, User, Yuque, YuqueError,
    YuqueResponse,
};

/// id - 仓库编号
/// type - 类型 [Book - 文档]
/// slug - 仓库路径
/// name - 名称
/// namespace - 仓库完整路径 user.login/book.slug
/// user_id - 所属的团队/用户编号
/// user - <UserSerializer>
/// description - 介绍
/// creator_id - 创建人 User Id
/// public - 公开状态 [1 - 公开, 0 - 私密]
/// likes_count - 喜欢数量
/// watches_count - 订阅数量
/// created_at - 创建时间
/// updated_at - 更新时间
#[derive(Deserialize, Debug)]
pub struct RepoListItem<'a> {
    pub id: i32,
    #[serde(rename = "type")]
    pub book_type: Cow<'a, str>,
    pub slug: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub namespace: Cow<'a, str>,
    pub user_id: i32,
    pub user: User<'a>,
    pub description: Option<Cow<'a, str>>,
    pub creator_id: i32,
    pub public: u8,
    pub likes_count: i32,
    pub watches_count: i32,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}

/// id - 仓库编号
/// type - 类型 [Book - 文档]
/// slug - 仓库路径
/// name - 名称
/// namespace - 仓库完整路径 user.login/book.slug
/// user_id - 所属的团队/用户编号
/// user - <UserSerializer>
/// description - 介绍
/// toc_yml - 目录原文
/// creator_id - 创建人 User Id
/// public - 公开状态 [1 - 公开, 0 - 私密]
/// items_count - 文档数量
/// likes_count - 喜欢数量
/// watches_count - 订阅数量
/// created_at - 创建时间
/// updated_at - 更新时间
#[derive(Debug, Deserialize)]
pub struct RepoDetail<'a> {
    pub id: i32,
    #[serde(rename = "type")]
    pub book_type: RepoType,
    pub slug: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub namespace: Cow<'a, str>,
    pub user_id: i32,
    pub user: User<'a>,
    pub description: Option<Cow<'a, str>>,
    #[serde(with = "toc_serde", rename = "toc_yml")]
    pub toc: Option<Toc<'a>>,
    pub creator_id: i32,
    pub public: u8,
    pub items_count: i32,
    pub likes_count: i32,
    pub watches_count: i32,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
pub enum RepoType {
    #[default]
    Book,
    Design,
    #[serde(rename = "all")]
    All,
}

/// * `name` - 仓库名称
/// * `slug` - 仓库路径
/// * `description` - 仓库介绍
/// * `public` - 公开状态 [2 - 成员公开, 1 - 公开, 0 - 私密]
/// * `book_type` - 仓库类型 [Book - 文档, Design - 设计]
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Repo {
    pub name: String,
    #[builder(default = "gen_random_slug(6)")]
    pub slug: String,
    #[builder(default = "String::new()")]
    pub description: String,
    #[builder(default = "1")]
    pub public: u8,
    #[builder(default = "RepoType::Book")]
    pub book_type: RepoType,
}

impl Repo {
    pub fn builder() -> RepoBuilder {
        RepoBuilder::default()
    }
}

impl<'a> TryFrom<RepoDetail<'a>> for Repo {
    type Error = YuqueError;

    fn try_from(value: RepoDetail<'a>) -> Result<Self, Self::Error> {
        Ok(Repo {
            name: value.name.into_owned(),
            slug: value.slug.into_owned(),
            description: value
                .description
                .map(|s| s.into_owned())
                .unwrap_or_default(),
            public: value.public,
            book_type: value.book_type,
        })
    }
}

pub struct ReposClient {
    pub(crate) client: Yuque,
}

impl ReposClient {
    /// List repo of user
    /// 获取用户的仓库列表
    ///
    /// # Arguments
    /// * `user` - 用户名/id
    /// * `data` - 查询参数
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let repos = yuque.repos().list_repo_of_user("username", None).await?;
    ///
    ///     println!("{:?}", repos);
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_repo_of_user(
        &self,
        user: impl ToString,
        data: Option<&[(&str, &str)]>,
    ) -> Result<YuqueResponse<Vec<RepoListItem>>, YuqueError> {
        let url = format!("/users/{}/repos", user.to_string());

        let data = data.unwrap_or_default();

        let response = self.client.get(&url)?.query(&data).send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// List repo of group
    /// 获取团队的仓库列表
    ///
    /// # Arguments
    /// * `group` - 团队名/id
    /// * `data` - 查询参数
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let repos = yuque.repos().list_repo_of_group("group name", None).await?;
    ///
    ///     println!("{:?}", repos);
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_repo_of_group(
        &self,
        group: impl ToString,
        data: Option<&[(&str, &str)]>,
    ) -> Result<YuqueResponse<Vec<RepoListItem>>, YuqueError> {
        let url = format!("/groups/{}/repos", group.to_string());

        let data = data.unwrap_or_default();

        let response = self.client.get(&url)?.query(&data).send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// create repo of user
    /// 创建用户的仓库
    ///
    /// # Arguments
    /// * `user` - 用户名/id
    /// * `data` - 仓库信息
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let repo = yuque.repos().create_repo_of_user("username", Repo::builder().name("test").build()).await?;
    ///
    ///     println!("{:?}", repo);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_repo_of_user(
        &self,
        user: impl ToString,
        data: Repo,
    ) -> Result<YuqueResponse<RepoDetail>, YuqueError> {
        let url = format!("/users/{}/repos", user.to_string());

        let data = serde_json::to_string(&data).ok();

        let response = self.client.post(&url, data)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// create repo of group
    /// 创建团队的仓库
    ///
    /// # Arguments
    /// * `group` - 团队名/id
    /// * `data` - 仓库信息
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///     
    ///     let repo = yuque.repos().create_repo_of_group("group name", Repo::builder().name("test").build()).await?;
    ///     
    ///     println!("{:?}", repo);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_repo_of_group(
        &self,
        group: impl ToString,
        data: Repo,
    ) -> Result<YuqueResponse<RepoDetail>, YuqueError> {
        let url = format!("/groups/{}/repos", group.to_string());

        let data = serde_json::to_string(&data).ok();

        let response = self.client.post(&url, data)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// get repo
    /// 获取仓库信息
    ///
    /// # Arguments
    /// * `repo` - 仓库名/id
    /// * `data` - 查询参数
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let repo = yuque.repos().get("username/repo name", None).await?;
    ///     
    ///     println!("{:?}", repo);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        repo: impl ToString,
        data: Option<&[(&str, &str)]>,
    ) -> Result<YuqueResponse<RepoDetail>, YuqueError> {
        let url = format!("/repos/{}", repo.to_string());

        let response = self.client.get(&url)?.query(&data).send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// update repo
    /// 更新仓库信息
    ///
    /// # Arguments
    /// * `repo` - 仓库名/id
    /// * `data` - 仓库信息
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let response = yuque.repos().update("username/repo name", Repo::builder().name("test").build()).await?;
    ///
    ///     println!("{:?}", response);
    ///     Ok(())
    /// }
    /// ```   
    pub async fn update(
        &self,
        repo: impl ToString,
        data: Repo,
    ) -> Result<YuqueResponse<RepoDetail>, YuqueError> {
        let url = format!("/repos/{}", repo.to_string());

        let data = serde_json::to_string(&data).ok();

        let response = self.client.put(&url, data)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// delete repo
    /// 删除仓库
    ///
    /// # Arguments
    /// * `repo` - 仓库名/id
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let response = yuque.repos().delete("username/repo name").await?;
    ///
    ///     println!("{:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, repo: impl ToString) -> Result<(), YuqueError> {
        let url = format!("/repos/{}", repo.to_string());

        let response = self.client.delete(&url)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(())
    }
}
