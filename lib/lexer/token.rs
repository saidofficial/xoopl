use nom::*;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    //Identifier and literals
    Ident(string),
    StringLiteral(String),
    IntLiteral(i64),
    BoolLiteral(bool),

    //Statements
    Assign,
    If,
    Else,

    //Operators
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
    Not,

    //Reserved words
    Function,
    Let,
    Return,

    //Punctuations
    Comma,
    Colon,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tok: &'a [Token],
    pub start: usize;
    pub end: usize,
}
