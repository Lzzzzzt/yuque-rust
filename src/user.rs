use std::borrow::Cow;

use serde::Deserialize;

use chrono::{DateTime, Local};

use crate::time_serde;

/// id - 用户编号
/// type - 类型 [`User`  - 用户, Group - 团队]
/// login - 用户个人路径
/// name - 昵称
/// avatar_url - 头像 URL
/// created_at - 创建时间
/// updated_at - 更新时间
#[derive(Deserialize, Debug)]
pub struct User<'a> {
    pub id: i32,
    #[serde(rename = "type")]
    pub user_type: Cow<'a, str>,
    pub login: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub avatar_url: Cow<'a, str>,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}

/// id - 用户资料编号
/// space_id - 企业空间编号
/// account_id - 用户账户编号
/// type - 类型 [User - 用户, Group - 团队]
/// login - 用户个人路径
/// name - 昵称
/// owner_id - 团队创建人，仅适用于 type - 'Group'
/// avatar_url - 头像 URL
/// books_count - 仓库数量
/// public_books_count - 公开仓库数量
/// members_count - 团队成员数量
/// description - 介绍
/// created_at - 创建时间
/// updated_at - 更新时间
#[derive(Deserialize, Debug)]
pub struct UserDetail<'a> {
    pub id: i32,
    pub space_id: i32,
    pub account_id: i32,
    #[serde(rename = "type")]
    pub user_type: Cow<'a, str>,
    pub login: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub owner_id: Option<i32>,
    pub avatar_url: Cow<'a, str>,
    pub books_count: i32,
    pub public_books_count: i32,
    pub members_count: i32,
    pub description: Option<Cow<'a, str>>,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}
