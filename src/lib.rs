use std::{borrow::Cow, fmt::Display};

use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use rand::Rng;
use reqwest::Method;

mod client;
mod docs;
mod error;
mod group;
mod repos;
mod response;
mod serde;
mod user;
use crate::serde::*;
pub use client::*;
pub use docs::*;
pub use error::*;
pub use group::*;
pub use repos::*;
pub use response::*;
pub use user::*;

pub const DEFAULT_USER_AGENT: &str = "@yuque/sdk";

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            RequestMethod::Get => "GET",
            RequestMethod::Post => "POST",
            RequestMethod::Put => "PUT",
            RequestMethod::Delete => "DELETE",
        };

        write!(f, "{}", string)
    }
}

impl From<RequestMethod> for Method {
    fn from(value: RequestMethod) -> Self {
        match value {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
            RequestMethod::Put => Method::PUT,
            RequestMethod::Delete => Method::DELETE,
        }
    }
}

pub(crate) fn judge_status_code(status_code: u16, url: String) -> Result<(), YuqueError> {
    match status_code {
        400 => Err(YuqueError::InvalidParams(url)),
        401 => Err(YuqueError::InvalidUserInfo(url)),
        403 => Err(YuqueError::NoPermission(url)),
        404 => Err(YuqueError::NotFound(url)),
        500 => Err(YuqueError::ServerException(url)),
        _ => Ok(()),
    }
}

pub(crate) fn gen_random_slug(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
pub enum YuqueFormat {
    #[serde(rename = "lake")]
    Lake,
    #[serde(rename = "markdown")]
    #[default]
    Markdown,
    #[serde(rename = "html")]
    Html,
}

impl From<YuqueFormat> for String {
    fn from(value: YuqueFormat) -> Self {
        match value {
            YuqueFormat::Lake => "lake".into(),
            YuqueFormat::Markdown => "markdown".into(),
            YuqueFormat::Html => "html".into(),
        }
    }
}

impl From<&YuqueFormat> for String {
    fn from(value: &YuqueFormat) -> Self {
        match value {
            YuqueFormat::Lake => "lake".into(),
            YuqueFormat::Markdown => "markdown".into(),
            YuqueFormat::Html => "html".into(),
        }
    }
}

impl From<YuqueFormat> for &str {
    fn from(value: YuqueFormat) -> Self {
        value.into()
    }
}

impl From<&YuqueFormat> for &str {
    fn from(value: &YuqueFormat) -> Self {
        value.into()
    }
}

impl Display for YuqueFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            YuqueFormat::Lake => "lake",
            YuqueFormat::Markdown => "markdown",
            YuqueFormat::Html => "html",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Toc<'a> {
    pub meta: TocMeta<'a>,
    pub toc: Vec<TocItem<'a>>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct TocMeta<'a> {
    #[serde(rename = "type")]
    item_type: Cow<'a, str>,
    count: u32,
    display_level: u32,
    tail_type: Cow<'a, str>,
    base_version_id: u32,
    published: bool,
    max_level: u32,
    #[serde(with = "time_serde")]
    last_updated_at: DateTime<Local>,
    version_id: u32,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(unused)]
pub struct TocItem<'a> {
    #[serde(rename = "type")]
    item_type: Cow<'a, str>,
    title: Cow<'a, str>,
    uuid: Cow<'a, str>,
    url: Cow<'a, str>,
    prev_uuid: Cow<'a, str>,
    sibling_uuid: Cow<'a, str>,
    child_uuid: Cow<'a, str>,
    parent_uuid: Cow<'a, str>,
    doc_id: u32,
    level: u32,
    id: u32,
    open_window: u32,
    visible: u32,
}
