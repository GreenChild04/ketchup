use crate::Span;
use core::fmt::Debug;

#[derive(Debug)]
pub struct TokenInfo<Oper> {
    /// the type of operation the token is
    pub oper: Oper,
    /// the location of the token
    pub span: Span,
    /// the amount of inputs the token/node takes
    pub space: u8,
    /// the precedence of the token
    pub precedence: u8,
}

#[derive(Debug)]
/// a single node in the `ASA`
pub struct Node<Token: Debug> {
    /// the interal type of the node
    pub token: Token,
    /// the location of that node's token
    pub span: Span,
    /// the parent of the node in the `ASA`
    pub parent: Option<usize>,
    /// the precedence of the node's token type in the `ASA`
    pub precedence: u8,
}

impl<Token: Debug> Node<Token> {
    /// creates a new node
    #[inline]
    pub fn new(token: Token, span: Span, parent: Option<usize>, precedence: u8) -> Self {
        Self {
            token,
            span,
            parent,
            precedence,
        }
    }
}
