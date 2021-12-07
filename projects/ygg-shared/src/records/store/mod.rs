mod lsp;

use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};

use ropey::Rope;
use url::Url;

#[derive(Default)]
pub struct TextStore {
    inner: DashMap<Url, Rope>,
}

impl TextStore {
    #[inline]
    pub fn force_update(&mut self, url: Url) -> Option<Rope> {
        todo!("{}", url)
    }

    pub fn insert(&mut self, url: Url, text: &str) -> Option<Rope> {
        self.inner.insert(url, Rope::from_str(text))
    }

    pub fn insert_inc(&mut self, url: Url, offset: usize, text: &str) {
        match self.inner.get_mut(&url) {
            Some(mut s) => {
                s.value_mut().insert(offset, text)
            }
            None => { todo!() }
        }
    }
    pub fn delete_inc(&mut self, url: Url) {
        match self.inner.get_mut(&url) {
            Some(_) => {
                todo!()
            }
            None => {}
        }
    }
}

impl TextStore {
    pub fn get_text(&self, url:&Url) -> Option<&str> {
       match  self.inner.get(url) {
           Some(_) => {todo!()},
           None => {None}
       }
    }
    pub fn get(){

    }

}