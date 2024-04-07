//! The **Abstract Syntax Array** is like the `AST` though optimised for cache locality through the usage of an array instead of pointers to memory

use crate::node::Node;
use core::fmt::Debug;

/// the **Abstract Syntax Array**, like the `AST` but in an array-like object
pub trait ASA: IntoIterator + Default + Debug {
    /// the token used within the array
    type Oper: Debug;

    /// pushes a node to the end of the `ASA`
    fn push(&mut self, node: Node<Self::Oper>);
    /// inserts a node into a position in the `ASA`
    fn insert(&mut self, idx: usize, node: Node<Self::Oper>);
    /// returns a node at a position in the `ASA`
    fn get(&mut self, idx: usize) -> &mut Node<Self::Oper>;
    /// returns the length of the `ASA`
    fn len(&self) -> usize;
}

/// a default implementation of `ASA` for a vector
impl<Token: Debug> ASA for Vec<Node<Token>> {
    type Oper = Token;

    #[inline]
    fn push(&mut self, node: Node<Self::Oper>) {
        self.push(node);
    }

    #[inline]
    fn insert(&mut self, idx: usize, node: Node<Self::Oper>) {
        self.insert(idx, node);
    }

    #[inline]
    fn get(&mut self, idx: usize) -> &mut Node<Self::Oper> {
        &mut self[idx]
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

/// # TODO
/// a padded version of an `ASA` vector for extra performance upon ownership changes
/// because a vector that has capacity at both the end and start of a vector allows for
/// items to be inserted at the start of the vector without having to shift everything to the right
pub struct PaddedVec<Token> {
    offset: usize,
    array: Box<[Token]>,
    len: usize,
}

impl<Token> PaddedVec<Token> {
    /// creates a new padded vector
    #[inline]
    pub fn new() -> Self {
        Self {
            offset: 0,
            array: Box::new([]),
            len: 0,
        }
    }

    // /// creates a new padded vector with a specified capacity
    // #[inline]
    // pub fn with_capacity(capacity: usize) -> Self {
    //     Self {
    //         offset: capacity / 2,
    //         array: Box::new([])
    //     }
    // }
}
