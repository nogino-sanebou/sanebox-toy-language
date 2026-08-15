use sanebox::*;

#[cfg(test)]
mod tests {
    use sanebox::*;

    // println関数の末尾括弧が閉じられていない
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
                assert_eq!(e.to_string(), "関数名が)で閉じられていません。token = Some(Semicolon)");
            },
        }
    }

    // println関数の末尾括弧が閉じられていない
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
                assert_eq!(e.to_string(), "関数名が)で閉じられていません。token = Some(Semicolon)");
            },
        }
    }

    // println関数の括弧の数が多い
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

    // println関数の括弧の数が多い
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

    // println関数にセミコロンがない
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

    // println関数にセミコロンがない
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

    // ゼロ除算
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

    // 計算式不備
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

    // 数値を返さない関数で計算
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

    // 数値を返さない関数で計算
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

    // 数値を返さない関数をabsの引数にした
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

    // 変数名だけを書いた
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

    // 変数と計算しようとした
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

    // abs関数に変数名を指定した
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

    // 存在しない関数を指定した
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

    // 変数宣言に右辺がない
    #[test]
    fn error_016() {
        let tokens = lexer("let x = ;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "予期せぬ値です。expr_primary = Semicolon");
            }
        }
    }

    // 変数宣言の右辺が数値を返さない関数
    #[test]
    fn error_017() {
        let tokens = lexer("let x = print(10 * 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let result = eval_all(expr);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数の値にUnitが出現しました。");
            }
        }
    }

    // letの後に変数名がない
    #[test]
    fn error_018() {
        let tokens = lexer("let = 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数名が文字列ではありません。token = Equal");
            }
        }
    }

    // 変数名の後に=がない
    #[test]
    fn error_019() {
        let tokens = lexer("let x 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数名の次は「=」でないといけません。token = Some(Number(10))");
            }
        }
    }

    // letの後に予約語がある
    #[test]
    fn error_020() {
        let tokens = lexer("let let = 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数名に予約語は使用できません。name = let");
            }
        }
    }
}