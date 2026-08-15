#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    Text(String),
    Number(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
    Equal,
}

pub fn lexer(code: &str) -> Vec<Token> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::LParen);
            },
            ')' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::RParen);
            },
            '+' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Plus);
            },
            '-' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Minus);
            },
            '*' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Asterisk);
            },
            '/' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Slash);
            },
            ';' => {
                push_literal(&mut tokens, &mut token);
                tokens.push(Token::Semicolon);
            },
            '=' => {
                push_literal(&mut  tokens, &mut token);
                tokens.push(Token::Equal);
            }
            c if c.is_whitespace() => {
                push_literal(&mut tokens, &mut token);
            },
            _ => {
                token.push(c);
            },
        }
    }

    push_literal(&mut tokens, &mut token);

    tokens
}

fn convert_literal(token: &str) -> Token {
    if let Ok(num) = token.parse::<i64>() {
        Token::Number(num)
    } else {
        if token.is_empty() {
            panic!("想定外の空文字が出現しました。")
        }
        Token::Text(token.to_string())
    }
}

fn push_literal(tokens: &mut Vec<Token>, token: &mut String) {
    if !token.is_empty() {
        tokens.push(convert_literal(&token));
        token.clear();
    }
}