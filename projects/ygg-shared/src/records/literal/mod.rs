mod traits;

use std::{
    cmp::Ordering,
    ops::{Deref, Range},
};

/// Maybe have ast position
pub type MaybeRanged = Option<Range<usize>>;

/// Used to represent a node with positions
#[derive(Clone)]
pub struct Literal<T> {
    /// The actual value
    pub value: T,
    /// The Start offset and end offset
    pub range: MaybeRanged,
}
