#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use nom::{IResult, bytes::complete::{tag, take_while_m_n}, branch::alt, character::{is_alphanumeric, is_digit}};
use std::str;

fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {
    let mut input = input.as_bytes();
    let mut tokens: Vec<Token> = Vec::new();
    input = trim_leading_whitespace(input);
    while input.len() > 0 {
        let formatted = str::from_utf8(input).unwrap();
        let token: Token;
        (input, token) = alt((
            lex_open_brace,
            lex_close_brace,
            lex_open_paren,
            lex_close_paren,
            lex_semicolon,
            lex_keyword,
            lex_integer_literal,
            lex_identifier,
        ))(input)?;
        tokens.push(token);
        input = trim_leading_whitespace(input);
    }
    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    Int(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Int,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keywords),
    Literal(Literals),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
}

pub type LexError<'a> = nom::Err<nom::error::Error<&'a [u8]>>;

fn trim_leading_whitespace(input: &[u8]) -> &[u8] {
    // TODO: Could use nom::take_while
    let mut i = 0;
    while i < input.len() && input[i].is_ascii_whitespace() {
        i += 1;
    }
    &input[i..]
}

fn lex_open_brace(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag("{")(input)?;
    Ok((input, Token::OpenBrace))
}

fn lex_close_brace(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag("}")(input)?;
    Ok((input, Token::CloseBrace))
}

fn lex_open_paren(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag("(")(input)?;
    Ok((input, Token::OpenParen))
}

fn lex_close_paren(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag(")")(input)?;
    Ok((input, Token::CloseParen))
}

fn lex_semicolon(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag(";")(input)?;
    Ok((input, Token::Semicolon))
}

fn lex_keyword(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, keyword) = alt((
        tag("int"),
        tag("return"),
    ))(input)?;

    let keyword = match keyword {
        b"int" => Keywords::Int,
        b"return" => Keywords::Return,
        _ => unreachable!(),
    };

    Ok((input, Token::Keyword(keyword)))
}

fn lex_identifier(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, mut identifier) = take_while_m_n(1, 32, is_alphanumeric)(input)?;
    let mut s = String::new();
    identifier.read_to_string(&mut s)
        .expect("failed to parse identifier; likely not UTF-8 encoded");
    Ok((input, Token::Identifier(s)))
}

fn lex_integer_literal(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, slice) = take_while_m_n(1, 16, is_digit)(input)?;
    let int = parse_int(slice);
    Ok((input, Token::Literal(Literals::Int(int))))
}

fn parse_int(input: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for d in input {
        result *= 10;
        result += (d - 48) as u32;
    }
    result
}



pub struct AST {
    root: 
}



#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn assert_lexes_to(src: &str, result: &Result<Vec<Token>, LexError>) {
        assert_eq!(lex(&src.into()), *result);
    }

    fn assert_file_lexes_to(path: &PathBuf, result: &Result<Vec<Token>, LexError>) {
        let mut file = File::open(path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        assert_lexes_to(&code, result);
    }

    fn test_file(path: &str) -> PathBuf {
        Path::new("/Users/suneelfreimuth/Code/c-compiler/write_a_c_compiler")
            .join(path)
    }

    #[test]
    fn test_empty_program() {
        assert_lexes_to("", &Ok(vec![]));
        assert_lexes_to("    ", &Ok(vec![]));
        assert_lexes_to("   \n   \n ", &Ok(vec![]));
    }

    #[test]
    fn test_symbolic() {
        assert_lexes_to("{", &Ok(vec![ OpenBrace ]));
        assert_lexes_to("}", &Ok(vec![ CloseBrace ]));
        assert_lexes_to("(", &Ok(vec![ OpenParen ]));
        assert_lexes_to(")", &Ok(vec![ CloseParen ]));
        assert_lexes_to(";", &Ok(vec![ Semicolon ]));
    }

    #[test]
    fn test_keywords() {
        assert_lexes_to("int", &Ok(vec![ Keyword(Keywords::Int) ]));
        assert_lexes_to("return", &Ok(vec![ Keyword(Keywords::Return) ]));
    }

    #[test]
    fn test_identifiers() {
        assert_lexes_to("x", &Ok(vec![ Identifier("x".into()) ]));
        assert_lexes_to("abc123", &Ok(vec![ Identifier("abc123".into()) ]));
    }

    #[test]
    fn test_integer_literals() {
        assert_lexes_to("1", &Ok(vec![ Literal(Literals::Int(1)) ]));
        assert_lexes_to("123", &Ok(vec![ Literal(Literals::Int(123)) ]));
        assert_lexes_to("666", &Ok(vec![ Literal(Literals::Int(666)) ]));
    }

    #[test]
    fn test_stage1_valid() {
        assert_file_lexes_to(&test_file("stage_1/valid/multi_digit.c"), &Ok(vec![
            Keyword(Keywords::Int),
            Identifier("main".into()),
            OpenParen,
            CloseParen,
            OpenBrace,
            Keyword(Keywords::Return),
            Literal(Literals::Int(100)),
            Semicolon,
            CloseBrace
        ]));

        assert_file_lexes_to(&test_file("stage_1/valid/return_2.c"), &Ok(vec![
            Keyword(Keywords::Int),
            Identifier("main".into()),
            OpenParen,
            CloseParen,
            OpenBrace,
            Keyword(Keywords::Return),
            Literal(Literals::Int(2)),
            Semicolon,
            CloseBrace
        ]));

        let return_0 = &Ok(vec![
            Keyword(Keywords::Int),
            Identifier("main".into()),
            OpenParen,
            CloseParen,
            OpenBrace,
            Keyword(Keywords::Return),
            Literal(Literals::Int(0)),
            Semicolon,
            CloseBrace
        ]);

        assert_file_lexes_to(&test_file("stage_1/valid/newlines.c"), &return_0);
        assert_file_lexes_to(&test_file("stage_1/valid/no_newlines.c"), &return_0);
        assert_file_lexes_to(&test_file("stage_1/valid/return_0.c"), &return_0);
        assert_file_lexes_to(&test_file("stage_1/valid/spaces.c"), &return_0);
    }
}