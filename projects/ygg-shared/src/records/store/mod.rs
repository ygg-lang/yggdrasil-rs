mod lsp;

use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use crate::Result;
use ropey::Rope;
use url::Url;
use std::ops::RangeBounds;
use crate::errors::Error;

#[derive(Default)]
pub struct TextStore {
    inner: DashMap<Url, Rope>,
}

impl TextStore {
    #[inline]
    pub fn force_update(&mut self, url: Url) -> Option<Rope> {
        todo!("{}", url)
    }
    #[inline]
    pub fn insert(&mut self, url: Url, text: &str) -> Option<Rope> {
        self.inner.insert(url, Rope::from_str(text))
    }

    pub fn insert_incremental(&mut self, url: Url, offset: usize, text: &str) -> Result<()> {
        match self.inner.get_mut(&url) {
            Some(mut s) => {
                Ok(s.value_mut().try_insert(offset, text)?)
            }
            None => {
                Err(Error::Unreachable)
            }
        }
    }
    #[inline]
    pub fn delete(&mut self, url: &Url) -> Option<(Url, Rope)> {
        self.inner.remove(url)
    }

    pub fn delete_incremental(&mut self, url: Url, range: impl RangeBounds<usize>) -> Result<()> {
        match self.inner.get_mut(&url) {
            Some(mut s) => {
                Ok(s.value_mut().try_remove(range)?)
            }
            None => { Err(Error::Unreachable) }
        }
    }
}

impl TextStore {
    #[inline]
    pub fn get_text(&self, url: &Url) -> Option<String> {
        self.inner.get(url).map(|f| String::from(f.value()))
    }
    pub fn get_text_indexed() {

    }
}