use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};
use std::ops::Range;

/// It's a cst_node contained in the Lossless Concrete Syntax Tree
/// All subsequent required information will be retained
/// Including spaces, line breaks, and comments or other semantically irrelevant content.
/// Macros and formatting can start at this level
pub struct CSTNode<Rule> {
    pub rule: Rule,
    pub range: Range<usize>,
    pub children: Vec<CSTNode<Rule>>,
    pub node_tag: Option<&'static str>,
    pub branch_tag: Option<&'static str>,
}

impl<R: Debug + Clone> Debug for CSTNode<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Node");
        w.field("rule", &self.rule);
        w.field("range", &format!("{}-{}", self.range.start, self.range.end));
        if let Some(s) = self.node_tag {
            w.field("label", &s);
        };
        if let Some(s) = self.branch_tag {
            w.field("branch", &s);
        };
        w.field("children", &self.children);
        w.finish()
    }
}

impl<R> CSTNode<R> {
    /// get str of the cst_node
    pub fn get_str<'i>(&self, input: &'i str) -> &'i str {
        unsafe { input.get_unchecked(self.range.start..self.range.end) }
    }
    /// Provide basic location information
    /// (start_offset, end_offset)
    pub fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
    /// Find node tags in all of the children
    /// Then collect them into a vec, and text_store in hashmap with the tag name
    pub fn get_tag_map(self) -> HashMap<&'static str, Vec<Self>> {
        let mut out: HashMap<&'static str, Vec<Self>> = HashMap::new();
        for node in self.children {
            if let Some(s) = node.node_tag {
                match out.get_mut(s) {
                    Some(s) => s.push(node),
                    None => {
                        out.insert(s, vec![node]);
                    }
                }
            }
        }
        return out;
    }
}
