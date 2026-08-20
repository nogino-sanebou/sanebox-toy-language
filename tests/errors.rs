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
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Semicolon");
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
                assert_eq!(e.to_string(), "addの右辺に数値以外が出現しました。");
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
                assert_eq!(e.to_string(), "addの左辺に数値以外が出現しました。");
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
                assert_eq!(e.to_string(), "abs関数に数値以外を指定しました。");
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
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Semicolon");
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

    // 定義されていない変数を書いた場合
    #[test]
    fn error_021() {
        let tokens = lexer("y;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "y は変数として宣言されていません。");
            }
        }
    }

    // 定義されていない変数を書いた場合
    // ただし、別の名前の変数が定義されている
    #[test]
    fn error_022() {
        let tokens = lexer("let x = 10; y;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "y は変数として宣言されていません。");
            }
        }
    }

    // 定義されていない変数を書いた場合
    // ただし、その後で同名の変数が定義されている
    #[test]
    fn error_023() {
        let tokens = lexer("x; let x = 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "x は変数として宣言されていません。");
            }
        }
    }


    // 右辺が数値でない
    #[test]
    fn error_024() {
        let tokens = lexer("10 < println(10); ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "lessの右辺に数値以外が出現しました。");
            }
        }
    }

    // 左辺が数値でない
    #[test]
    fn error_025() {
        let tokens = lexer("println(10) < 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "lessの左辺に数値以外が出現しました。");
            }
        }
    }

    // 比較演算子が連続している
    #[test]
    fn error_026() {
        let tokens = lexer("10 <> 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Greater");
            }
        }
    }

    // 次のトークンがマイナスでない算術演算子
    #[test]
    fn error_027() {
        let tokens = lexer("10 < + 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Plus");
            }
        }
    }

    // 右辺が数値でない
    #[test]
    fn error_028() {
        let tokens = lexer("10 > println(10); ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "greaterの右辺に数値以外が出現しました。");
            }
        }
    }

    // 左辺が数値でない
    #[test]
    fn error_029() {
        let tokens = lexer("println(10) > 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "greaterの左辺に数値以外が出現しました。");
            }
        }
    }

    // 比較演算子が連続している
    #[test]
    fn error_30() {
        let tokens = lexer("10 >< 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Less");
            }
        }
    }

    // 次のトークンがマイナスでない算術演算子
    #[test]
    fn error_031() {
        let tokens = lexer("10 > + 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Plus");
            }
        }
    }

    // 右辺が数値でない
    #[test]
    fn error_032() {
        let tokens = lexer("10 == println(10); ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "equalの右辺に数値以外が出現しました。");
            }
        }
    }

    // 左辺が数値でない
    #[test]
    fn error_033() {
        let tokens = lexer("println(10) == 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "equalの左辺に数値以外が出現しました。");
            }
        }
    }

    // 比較演算子が連続している
    #[test]
    fn error_34() {
        let tokens = lexer("10 ==== 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = EqualEqual");
            }
        }
    }

    // 次のトークンがマイナスでない算術演算子
    #[test]
    fn error_035() {
        let tokens = lexer("10 == + 10; ");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Plus");
            }
        }
    }

    // let 変数名の次が無い場合
    #[test]
    fn error_036() {
        let tokens = lexer("let x =");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "letの初期値に値が存在しませんでした。");
            }
        }
    }

    // 数値 < 数値 < 数値のパターン
    #[test]
    fn error_037() {
        let tokens = lexer("10 < 20 < 30;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "lessの左辺に数値以外が出現しました。");
            }
        }
    }

    // 数値 < = 数値のパターン
    #[test]
    fn error_038() {
        let tokens = lexer("10 < = 20;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Equal");
            }
        }
    }

    // 数値 > = 数値のパターン
    #[test]
    fn error_039() {
        let tokens = lexer("10 > = 20;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "expr_primaryで予期せぬトークンが出現しました。token = Equal");
            }
        }
    }

    // 数値 ! = 数値のパターン
    // !(boolean); を実装するまでエラー内容は仮置きで「式の末尾がセミコロンでありません。」にする
    #[test]
    fn error_040() {
        let tokens = lexer("10 ! = 20;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            }
        }
    }

    // 数値 = = 数値のパターン
    // 変数 = 数値; を実装するまでエラー内容は仮置きで「式の末尾がセミコロンでありません。」にする
    #[test]
    fn error_041() {
        let tokens = lexer("10 = = 20;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "式の末尾がセミコロンでありません。");
            }
        }
    }

    // 数値 <= 戻り値がUnit型の関数
    #[test]
    fn error_042() {
        let tokens = lexer("10 <= println(1 + 2);");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "less_equalの右辺に数値以外が出現しました。");
            }
        }
    }

    // 戻り値がUnit型の関数 <= 数値
    #[test]
    fn error_043() {
        let tokens = lexer("println(1 + 2) <= 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "less_equalの左辺に数値以外が出現しました。");
            }
        }
    }

    // 数値 >= 戻り値がUnit型の関数
    #[test]
    fn error_044() {
        let tokens = lexer("10 >= println(1 + 2);");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "greater_equalの右辺に数値以外が出現しました。");
            }
        }
    }

    // 戻り値がUnit型の関数 >= 数値
    #[test]
    fn error_045() {
        let tokens = lexer("println(1 + 2) >= 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "greater_equalの左辺に数値以外が出現しました。");
            }
        }
    }

    // 数値 != 戻り値がUnit型の関数
    #[test]
    fn error_046() {
        let tokens = lexer("10 != println(1 + 2);");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "not_equalの右辺に数値以外が出現しました。");
            }
        }
    }

    // 戻り値がUnit型の関数 != 数値
    #[test]
    fn error_047() {
        let tokens = lexer("println(1 + 2) != 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "not_equalの左辺に数値以外が出現しました。");
            }
        }
    }

    // abs(boolean)のパターン
    #[test]
    fn error_048() {
        let tokens = lexer("abs(true);");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "abs関数に数値以外を指定しました。");
            }
        }
    }

    // 計算式にbooleanのパターン
    #[test]
    fn error_049() {
        let tokens = lexer("1 + false;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "addの右辺に数値以外が出現しました。");
            }
        }
    }

    // 比較演算子にbooleanのパターン
    #[test]
    fn error_050() {
        let tokens = lexer("10 <= true;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "less_equalの右辺に数値以外が出現しました。");
            }
        }
    }

    // 変数名にtrueを指定した場合
    #[test]
    fn error_051() {
        let tokens = lexer("let true = 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数名に予約語は使用できません。name = true");
            }
        }
    }

    // 変数名にfalseを指定した場合
    #[test]
    fn error_052() {
        let tokens = lexer("let false = 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse();

        match stmt {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "変数名に予約語は使用できません。name = false");
            }
        }
    }

    // 論理演算子(&&)にbool以外を指定した場合
    #[test]
    fn error_053() {
        let tokens = lexer("100 && true;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "andの左辺にboolean以外が出現しました。");
            }
        }
    }

    // 論理演算子(||)にbool以外を指定した場合
    #[test]
    fn error_054() {
        let tokens = lexer("false || 100;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();
        let result = eval_all(stmt);

        match result {
            Ok(_) => {
                panic!("エラーになるべき入力が成功しました。");
            },
            Err(e) => {
                assert_eq!(e.to_string(), "orの右辺にboolean以外が出現しました。");
            }
        }
    }
}