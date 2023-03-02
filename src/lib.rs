use std::fmt::Display;

use ::serde::{Deserialize, Serialize};
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
pub use crate::serde::*;
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
