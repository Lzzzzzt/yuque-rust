#![allow(unused)]

use std::{borrow::Cow, slice::Iter, vec::IntoIter};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct YuqueResponse<D> {
    pub data: D,
    pub abilities: Option<Abilities>,
}

impl<D> YuqueResponse<Vec<D>> {
    pub fn iter(&self) -> Iter<D> {
        self.data.iter()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntoIter<D> {
        self.data.into_iter()
    }
}

#[derive(Deserialize, Debug)]
pub struct Abilities {
    update: bool,
    destroy: bool,
}


