use crate::{
    frontend::{GrammarInfo, Keys, Map, Rule, Values},
    manager::HintItems,
};
use convert_case::{Case, Casing};
use lsp_types::{Range, Url};
use std::mem::swap;
use yggdrasil_bootstrap::{
    ast::{AssignStatement, Program, Statement, Symbol},
    shared::records::PositionSystem,
    Result,
};

pub use self::{
    from_ast::{GrammarContext, Translator},
    node::*,
};

mod from_ast;
mod hints;
mod node;

impl GrammarInfo {
    #[inline]
    pub fn get(&self, rule: &str) -> Option<&Rule> {
        self.rule_map.get(rule)
    }
    pub fn get_inline(&self, rule: &str) -> Option<&Rule> {
        match self.rule_map.get(rule) {
            Some(s) => Some(s),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn get_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        self.rule_map.get_mut(rule)
    }
    #[inline]
    pub fn get_inline_mut(&mut self, rule: &str) -> Option<&mut Rule> {
        match self.rule_map.get(rule) {
            Some(_) => self.rule_map.get_mut(rule),
            // Check manual inlining
            None if rule.starts_with("_") => self.rule_map.get_mut(&rule[1..=rule.len()]),
            _ => None,
        }
    }
    #[inline]
    pub fn insert(&mut self, name: String, rule: Rule) -> Option<Rule> {
        self.rule_map.insert(name, rule)
    }
    #[inline]
    pub fn keys(&self) -> Keys<String, Rule> {
        self.rule_map.keys()
    }
    #[inline]
    pub fn rules(&self) -> Values<String, Rule> {
        self.rule_map.values()
    }
    #[inline]
    pub fn named_rules(&self) -> Vec<Rule> {
        self.rule_map.values().cloned().filter(|r| !r.force_inline).collect()
    }
}

impl Rule {
    pub fn build_structure(&self) -> String {
        unimplemented!()
    }
    pub fn build_parse(&self) -> String {
        unimplemented!()
    }
    pub fn build_error(&self) -> String {
        unimplemented!()
    }
}
