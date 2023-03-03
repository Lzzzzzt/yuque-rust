use std::borrow::Cow;

use chrono::{DateTime, Local};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    gen_random_slug, judge_status_code, number_to_bool, option_time_serde, time_serde,
    RepoListItem, User, Yuque, YuqueError, YuqueFormat, YuqueResponse,
};

/// 文档列表项
///
/// # Fields
/// * `id: i32` - 文档编号
/// * `slug: Cow<'a, str>` - 文档路径
/// * `title: Cow<'a, str>` - 标题
/// * `description: Option<Cow<'a, str>>` - 描述
/// * `user_id: i32` - 文档创建人 user_id
/// * `format: YuqueFormat` - 描述了正文的格式 [asl, markdown]
/// * `public: bool` - 是否公开 [1 - 公开, 0 - 私密]
/// * `status: bool` - 状态 [1 - 正常, 0 - 草稿]
/// * `likes_count: u16` - 喜欢数量
/// * `comments_count: u16` - 评论数量
/// * `content_updated_at: Option<DateTime<Local>>` - 文档内容更新时间
/// * `book: Repo<'a>` - <Repo> 所属知识库
/// * `user: User<'a>` - <User> 所属团队（个人）
/// * `last_editor: User<'a>` - <User> 最后修改人
/// * `created_at: DateTime<Local>` - 创建时间
/// * `updated_at: DateTime<Local>` - 更新时间
#[derive(Deserialize, Debug)]
pub struct DocListItem<'a> {
    pub id: i32,
    pub slug: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub user_id: i32,
    pub format: YuqueFormat,
    #[serde(with = "number_to_bool")]
    pub public: bool,
    #[serde(with = "number_to_bool")]
    pub status: bool,
    pub likes_count: u16,
    pub comments_count: u16,
    #[serde(with = "time_serde")]
    pub content_updated_at: DateTime<Local>,
    pub book: Option<RepoListItem<'a>>,
    pub user: Option<User<'a>>,
    pub last_editor: User<'a>,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}

/// DocDetail
/// 文档详情
///
/// # Fields
/// * `id: i32` - 文档编号
/// * `slug: Cow<'a, str>` - 文档路径
/// * `title: Cow<'a, str>` - 标题
/// * `book_id: i32` - 仓库编号，就是 repo_id
/// * `book: Option<Repo<'a>>` - 仓库信息 <Repo>，就是 repo 信息
/// * `user_id: i32` - 用户/团队编号
/// * `user: Option<User<'a>>` - 用户/团队信息 <User>
/// * `format: YuqueFormat` - 描述了正文的格式 [lake , markdown]
/// * `body: Cow<'a, str>` - 正文 Markdown 源代码
/// * `body_draft: Cow<'a, str>` - 草稿 Markdown 源代码
/// * `body_html: Cow<'a, str>` - 转换过后的正文 HTML （重大变更，详情请参考：https://www.yuque.com/yuque/developer/yr938f）
/// * `body_lake: Cow<'a, str>` - 语雀 lake 格式的文档内容
/// * `creator_id: i32` - 文档创建人 User Id
/// * `public: bool` - 公开级别 [0 - 私密, 1 - 公开]
/// * `status: bool` - 状态 [0 - 草稿, 1 - 正常]
/// * `likes_count: u16` - 喜欢数量
/// * `comments_count: u16` - 评论数量
/// * `content_updated_at: DateTime<Local>` - 文档内容更新时间
/// * `deleted_at: Option<DateTime<Local>>` - 删除时间，未删除为 null
/// * `created_at: DateTime<Local>` - 创建时间
/// * `updated_at: DateTime<Local>` - 更新时间
#[derive(Deserialize, Debug)]
pub struct DocDetail<'a> {
    pub id: i32,
    pub slug: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub book_id: i32,
    pub book: Option<RepoListItem<'a>>,
    pub user_id: i32,
    pub user: Option<User<'a>>,
    pub format: YuqueFormat,
    pub body: Cow<'a, str>,
    pub body_draft: Cow<'a, str>,
    pub body_html: Option<Cow<'a, str>>,
    pub body_lake: Option<Cow<'a, str>>,
    pub creator_id: Option<i32>,
    #[serde(with = "number_to_bool")]
    pub public: bool,
    #[serde(with = "number_to_bool")]
    pub status: bool,
    pub likes_count: Option<u16>,
    pub comments_count: Option<u16>,
    #[serde(with = "time_serde")]
    pub content_updated_at: DateTime<Local>,
    #[serde(with = "option_time_serde")]
    pub deleted_at: Option<DateTime<Local>>,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}

/// 用于post的文档
///
/// # Fields
///
/// * `title: String` - 标题
/// * `slug: String` - 文档 Slug
/// * `format: YuqueFormat` - 支持 markdown、lake、html，默认为 markdown
/// * `body: String` - format 描述的正文内容，最大允许 5MB
#[derive(Builder, Serialize, Deserialize, Clone, Default, Debug)]
pub struct Doc {
    pub title: String,
    #[builder(default = "gen_random_slug(16)")]
    pub slug: String,
    #[builder(default)]
    pub format: YuqueFormat,
    #[builder(default)]
    pub body: String,
}

impl Doc {
    /// 创建一个文档
    pub fn builder() -> DocBuilder {
        DocBuilder::default()
    }
}

impl<'a> TryFrom<DocDetail<'a>> for Doc {
    type Error = YuqueError;

    fn try_from(value: DocDetail<'a>) -> Result<Self, Self::Error> {
        match value.format {
            YuqueFormat::Markdown => (),
            _ => return Err(YuqueError::NotSupportFormat(value.format.into())),
        }

        Ok(Doc {
            title: value.title.into_owned(),
            slug: value.slug.into_owned(),
            format: value.format,
            body: value.body.into_owned(),
        })
    }
}

impl<'a> TryFrom<DocDetail<'a>> for (Doc, i32) {
    type Error = YuqueError;

    fn try_from(value: DocDetail<'a>) -> Result<Self, Self::Error> {
        match value.format {
            YuqueFormat::Markdown => (),
            _ => return Err(YuqueError::NotSupportFormat(value.format.into())),
        }

        Ok((
            Doc {
                title: value.title.into_owned(),
                slug: value.slug.into_owned(),
                format: value.format,
                body: value.body.into_owned(),
            },
            value.id,
        ))
    }
}

#[derive(Debug)]
pub struct DocsClient {
    pub(crate) client: Yuque,
}

impl DocsClient {
    /// List the documents of a repository
    /// 获取仓库下的文档列表
    ///
    /// # Arguments
    /// * `namespace: impl ToString` - 仓库的命名空间/id
    ///
    /// # Example
    /// ```rust
    ///
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("your token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let docs = yuque.docs().list_by_repo_namespace("your namespace").await?;
    ///
    ///     println!("{:?}", docs);
    ///     Ok(())
    /// }
    ///
    /// ```
    pub async fn list_with_repo(
        &self,
        namespace: impl ToString,
    ) -> Result<YuqueResponse<Vec<DocListItem>>, YuqueError> {
        let url = format!("/repos/{}/docs", namespace.to_string());

        let response = self.client.get(&url)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// Get a document
    /// 获取文档详情
    ///
    /// # Arguments
    /// * `namespace: impl ToString` - 仓库的命名空间
    /// * `slug: impl Into<String>` - 文档的 Slug
    /// * `data: Option<Vec<(String, String)>>` - 查询参数
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("your token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let doc = yuque.docs().get_with_repo_ns("your namespace", "your slug", None).await?;
    ///
    ///     println!("{:?}", doc);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_with_repo_ns(
        &self,
        namespace: impl ToString,
        slug: impl Into<String>,
        data: Option<&[(&str, &str)]>,
    ) -> Result<YuqueResponse<DocDetail>, YuqueError> {
        let url = format!("/repos/{}/docs/{}", namespace.to_string(), slug.into());

        let data = data.unwrap_or_default();

        let response = self.client.get(&url)?.query(&data).send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// Create a document
    /// 创建文档
    ///
    /// # Arguments
    /// * `namespace: impl ToString` - 仓库的命名空间/id
    /// * `data: Option<Doc>` - 文档数据
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::{Yuque, Doc};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("your token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let doc = Doc::builder()
    ///                     .title("title")
    ///                     .body("body")
    ///                     .build();
    ///
    ///     let doc = yuque.docs().create_with_repo("your namespace", Some(doc)).await?;
    ///
    ///     println!("{:?}", doc);
    ///     Ok(())
    /// }
    pub async fn create_with_repo(
        &self,
        namespace: impl ToString,
        data: Doc,
    ) -> Result<YuqueResponse<DocDetail>, YuqueError> {
        let url = format!("/repos/{}/docs", namespace.to_string());

        let data = serde_json::to_string(&data).ok();

        let response = self.client.post(&url, data)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// delete a document
    /// 删除文档
    ///
    /// # Arguments
    /// * `namespace: impl Into<String>` - 仓库的命名空间/id
    /// * `slug: impl Into<String>` - 文档的 Slug
    ///
    /// # Example
    ///
    /// ```rust
    /// use yuque_rust::Yuque;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("your token".to_string())
    ///                         .host("https://www.yuque.com".to_string())
    ///                         .build()?;
    ///
    ///     let doc = yuque.docs().delete_with_repo("your namespace", "your slug").await?;
    ///
    ///     println!("{:?}", doc);
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_with_repo(
        &self,
        namespace: impl ToString,
        id: i32,
    ) -> Result<YuqueResponse<DocDetail>, YuqueError> {
        let url = format!("/repos/{}/docs/{}", namespace.to_string(), id);

        let response = self.client.delete(&url)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }

    /// Update a document
    /// 更新文档
    ///
    /// # Arguments
    /// * `namespace: impl Into<String>` - 仓库的命名空间/id
    /// * `slug: impl Into<String>` - 文档的 Slug
    /// * `data: Option<Doc>` - 文档数据
    ///
    /// # Example
    /// ```rust
    /// use yuque_rust::{Yuque, Doc};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let yuque = Yuque::builder()
    ///                         .token("your token".to_string())
    ///                         .host("https://www.yuque.com".to_string())  
    ///                         .build()?;
    ///     
    ///     let doc = Doc::builder()
    ///                    .title("title")
    ///                     .body("body")
    ///                     .build();
    ///
    ///     let doc = yuque.docs().update_with_repo("your namespace", "doc id", Some(doc)).await?;
    ///
    ///     println!("{:?}", doc);
    ///     Ok(())
    /// }
    pub async fn update_with_repo(
        &self,
        namespace: impl ToString,
        id: i32,
        data: Doc,
    ) -> Result<YuqueResponse<DocDetail>, YuqueError> {
        match data.format {
            YuqueFormat::Markdown => (),
            _ => return Err(YuqueError::NotSupportFormat(data.format.into())),
        }

        let url = format!("/repos/{}/docs/{}", namespace.to_string(), id);

        let data = serde_json::to_string(&data).ok();

        let response = self.client.put(&url, data)?.send().await?;

        judge_status_code(response.status().as_u16(), url)?;

        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, ops::Not};

    use crate::{Doc, Yuque};

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    const TEST_NS: &str = "lzzzt/sdk-test";
    const TEST_HOST: &str = "https://lzzzt.yuque.com/api/v2";

    #[test]
    fn should_list_docs() -> Result<(), Box<dyn Error>> {
        dotenv::from_path(".env.dev").ok();

        let token = std::env::var("TOKEN")?;

        let client = Yuque::builder()
            .token(token)
            .host(TEST_HOST.into())
            .build()?
            .docs();

        let docs = aw!(client.list_with_repo(TEST_NS))?;

        assert!(docs.data.is_empty().not());

        let count = docs
            .into_iter()
            .map(|doc| doc.title)
            .filter(|title| title.to_ascii_lowercase().contains("test"))
            .count();

        assert!(count > 0);

        Ok(())
    }

    #[test]
    fn should_get_doc_detail() -> Result<(), Box<dyn Error>> {
        dotenv::from_path(".env.dev").ok();

        let token = std::env::var("TOKEN")?;

        let client = Yuque::builder()
            .token(token)
            .host(TEST_HOST.into())
            .build()?
            .docs();

        let doc =
            aw!(client.get_with_repo_ns(TEST_NS, "create-by-sdk", Some(&[("raw", "1")])))?.data;

        assert!(doc
            .body
            .contains("This sentence is created by yuque-rust sdk."));

        Ok(())
    }

    #[test]
    fn should_create_then_delete() -> Result<(), Box<dyn Error>> {
        dotenv::from_path(".env.dev").ok();

        let token = std::env::var("TOKEN")?;

        let client = Yuque::builder()
            .token(token)
            .host(TEST_HOST.into())
            .build()?
            .docs();

        let doc = Doc::builder()
            .title("Create By SDK".into())
            .body("Should be delete!".into())
            .slug("by-sdk".into())
            .build()?;

        let created_doc = aw!(client.create_with_repo(TEST_NS, doc.clone()))?.data;

        assert_eq!(doc.title, created_doc.title);
        assert!(created_doc.body.contains(doc.body.as_str()));
        assert_eq!(doc.slug, created_doc.slug);

        let deleted_doc = aw!(client.delete_with_repo(TEST_NS, created_doc.id))?.data;

        assert_eq!(doc.title, deleted_doc.title);

        Ok(())
    }

    #[test]
    fn should_update() -> Result<(), Box<dyn Error>> {
        dotenv::from_path(".env.dev").ok();

        let token = std::env::var("TOKEN")?;

        let client = Yuque::builder()
            .token(token)
            .host(TEST_HOST.into())
            .build()?
            .docs();

        let (mut doc, id): (Doc, i32) =
            aw!(client.get_with_repo_ns(TEST_NS, "create-by-sdk", Some(&[("raw", "1")])))?
                .data
                .try_into()?;

        let new_body = doc.body + &format!("\nLast Update: {}.", chrono::Local::now().to_rfc2822());

        doc.body = new_body.clone();

        let updated_doc = aw!(client.update_with_repo(TEST_NS, id, doc))?.data;

        assert_eq!(updated_doc.body, new_body);

        Ok(())
    }
}
