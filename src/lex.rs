use std::io::Read;

use nom::{IResult, bytes::complete::{tag, take_while_m_n}, branch::alt, character::{is_alphanumeric, is_digit}};
use std::str;

pub fn lex(input: &String) -> Result<Vec<Token>, LexError> {
    let mut input = input.as_bytes();
    let mut tokens: Vec<Token> = Vec::new();
    while input.len() > 0 {
        input = eat_whitespace(input);
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
        println!("{token:?}");
        tokens.push(token);
    }
    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
}

pub type LexError<'a> = nom::Err<nom::error::Error<&'a [u8]>>;

fn eat_whitespace(input: &[u8]) -> &[u8] {
    // TODO: Could use nom::take_while
    let mut i = 0;
    while input[i].is_ascii_whitespace() {
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
        b"int" => Keyword::Int,
        b"return" => Keyword::Return,
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
    Ok((input, Token::Literal(Literal::Int(int))))
}

fn parse_int(input: &[u8]) -> u32 {
    let mut result: u32 = 0;
    for d in input {
        result *= 10;
        result += (d - 48) as u32;
    }
    result
}

fn assert_lexes_to(src: &str, result: Result<Vec<Token>, LexError>) {
    assert_eq!(lex(&src.into()), result);
}

#[test]
fn test_symbolic() {
    assert_lexes_to("{", Ok(vec![ Token::OpenBrace ]));
    assert_lexes_to("}", Ok(vec![ Token::CloseBrace ]));
    assert_lexes_to("(", Ok(vec![ Token::OpenParen ]));
    assert_lexes_to(")", Ok(vec![ Token::CloseParen ]));
    assert_lexes_to(";", Ok(vec![ Token::Semicolon ]));
}

#[test]
fn test_keywords() {
    assert_lexes_to("int", Ok(vec![ Token::Keyword(Keyword::Int) ]));
    assert_lexes_to("return", Ok(vec![ Token::Keyword(Keyword::Return) ]));
}

#[test]
fn test_identifiers() {
    assert_lexes_to("x", Ok(vec![ Token::Identifier("x".into()) ]));
    assert_lexes_to("abc123", Ok(vec![ Token::Identifier("abc123".into()) ]));
}

#[test]
fn test_integer_literals() {
    assert_lexes_to("1", Ok(vec![ Token::Literal(Literal::Int(1)) ]));
    assert_lexes_to("123", Ok(vec![ Token::Literal(Literal::Int(123)) ]));
    assert_lexes_to("666", Ok(vec![ Token::Literal(Literal::Int(666)) ]));
}

const program: &str = "
int main() {
    return 2;
}
";

#[test]
fn test_whole_thing() {
    assert_lexes_to(program, Ok(vec![
        Token::Keyword(Keyword::Int),
        Token::Identifier("main".into()),
        Token::OpenParen,
        Token::CloseParen,
        Token::OpenBrace,
        Token::Keyword(Keyword::Return),
        Token::Literal(Literal::Int(2)),
        Token::Semicolon,
        Token::CloseBrace
    ]));
}