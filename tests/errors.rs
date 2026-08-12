use sanebox::{lexer, Parser};

#[cfg(test)]
mod tests {
    use sanebox::*;

    #[test]
    fn error_001() {
        let tokens = lexer("println((1 + 2 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "関数名が)で閉じられていません。");
            },
        }
    }

    #[test]
    fn error_002() {
        let tokens = lexer("println(1 + (2 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "関数名が)で閉じられていません。");
            },
        }
    }

    #[test]
    fn error_003() {
        let tokens = lexer("println(1 + 2));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            },
        }
    }

    #[test]
    fn error_004() {
        let tokens = lexer("println((1 + 2)));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            },
        }
    }

    #[test]
    fn error_005() {
        let tokens = lexer("println(1 + 2 + 3)");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            },
        }
    }

    #[test]
    fn error_006() {
        let tokens = lexer("1 + 2 - 3");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            },
        }
    }

    #[test]
    fn error_007() {
        let tokens = lexer("10 / 0;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        let eval = eval_all(expr.unwrap());

        match eval {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "0で除算できません。");
            },
        }
    }

    #[test]
    fn error_008() {
        let tokens = lexer("10 / ;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "予期せぬ値です。expr_primary = Semicolon");
            },
        }
    }

    #[test]
    fn error_009() {
        let tokens = lexer("1 + println(-10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr);

        match r {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "想定外のvalue add-rhs");
            }
        }
    }

    #[test]
    fn error_010() {
        let tokens = lexer("print(-10 + 1) + 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr);

        match r {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "想定外のvalue add-lhs");
            }
        }
    }

    #[test]
    fn error_011() {
        let tokens = lexer("abs(print(-10 + 1));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr);

        match r {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "数値以外が出現しました。Abs");
            }
        }
    }

    #[test]
    fn error_012() {
        let tokens = lexer("x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "現在は変数参照に対応していません。name = x");
            }
        }
    }

    #[test]
    fn error_013() {
        let tokens = lexer("x + 1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "現在は変数参照に対応していません。name = x");
            }
        }
    }

    #[test]
    fn error_014() {
        let tokens = lexer("abs(x);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "現在は変数参照に対応していません。name = x");
            }
        }
    }

    #[test]
    fn error_015() {
        let tokens = lexer("max(x, y);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "存在しない関数です。name = max");
            }
        }
    }
}