use std::borrow::Cow;

use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::{number_to_bool, time_serde, User};

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
pub struct Repo<'a> {
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
    #[serde(with = "number_to_bool")]
    pub public: bool,
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
    pub book_type: Cow<'a, str>,
    pub slug: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub namespace: Cow<'a, str>,
    pub user_id: i32,
    pub user: User<'a>,
    pub description: Option<Cow<'a, str>>,
    pub toc_yml: Option<Cow<'a, str>>,
    pub creator_id: i32,
    #[serde(with = "number_to_bool")]
    pub public: bool,
    pub items_count: i32,
    pub likes_count: i32,
    pub watches_count: i32,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}
