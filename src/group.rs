use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::{time_serde, User};

/// id - GroupUser Id
/// group_id - 团队编号
/// group - 团队信息 <UserSerializer>
/// user_id - 用户编号
/// user - 用户信息 <UserSerializer>
/// role - 角色 [0 - Owner, 1 - Member]
/// created_at - 创建时间
/// updated_at - 更新时间
#[derive(Debug, Deserialize)]
pub struct GroupUser<'a> {
    pub id: i32,
    pub group_id: i32,
    pub group: User<'a>,
    pub user_id: i32,
    pub user: User<'a>,
    pub role: i32,
    #[serde(with = "time_serde")]
    pub created_at: DateTime<Local>,
    #[serde(with = "time_serde")]
    pub updated_at: DateTime<Local>,
}
