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
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    Equal,
    Exclamation,
    ExclamationEqual,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
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
            '<' => {
                push_literal(&mut tokens, &mut token);
                // 次の文字が'='であった場合、LessEqualにする
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::LessEqual);
                    chars.next();
                }
                // 違った場合はLessにする
                else {
                    tokens.push(Token::Less);
                }
            },
            '>' => {
                push_literal(&mut tokens, &mut token);
                // 次の文字が'='であった場合、GreaterEqualにする
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::GreaterEqual);
                    chars.next();
                }
                // 違った場合はGreaterにする
                else {
                    tokens.push(Token::Greater);
                }
            },
            '=' => {
                push_literal(&mut  tokens, &mut token);
                // 次の文字も'='であった場合、EqualEqualにする
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::EqualEqual);
                    chars.next();
                }
                // 違った場合はEqualにする
                else {
                    tokens.push(Token::Equal);
                }
            },
            '!' => {
                push_literal(&mut  tokens, &mut token);
                // 次の文字が'='であった場合、ExclamationEqualにする
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::ExclamationEqual);
                    chars.next();
                }
                // 違った場合はExclamationにする
                else {
                    tokens.push(Token::Exclamation);
                }
            },
            '&' => {
                push_literal(&mut  tokens, &mut token);
                // 次の文字も'&'であった場合、AmpersandAmpersandにする
                if chars.peek() == Some(&'&') {
                    tokens.push(Token::AmpersandAmpersand);
                    chars.next();
                }
                // 違った場合はAmpersandにする
                else {
                    tokens.push(Token::Ampersand);
                }
            },
            '|' => {
                push_literal(&mut  tokens, &mut token);
                // 次の文字も'|'であった場合、PipePipeにする
                if chars.peek() == Some(&'|') {
                    tokens.push(Token::PipePipe);
                    chars.next();
                }
                // 違った場合はPipeにする
                else {
                    tokens.push(Token::Pipe);
                }
            },
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

fn push_literal(tokens: &mut Vec<Token>, token: &mut String) {
    if !token.is_empty() {
        tokens.push(convert_literal(&token));
        token.clear();
    }
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
