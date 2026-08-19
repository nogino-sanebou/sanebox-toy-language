#[cfg(test)]
mod tests {
    use sanebox::*;

    // 複数ステートメント
    #[test]
    fn normal_001() {
        let tokens = lexer("1 + 2; 3 + 4 - 5; -6 * 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(3),
                Value::Number(2),
                Value::Number(-12),
            ]
        );
    }

    // 複数ステートメント(関数あり)
    #[test]
    fn normal_002() {
        let tokens = lexer("(1 + 2) * (3 + 4); print(5 + 5); print(2 * 2 * 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(21),
                Value::Unit,
                Value::Unit,
            ]
        );
    }

    // 複数ステートメント(改行あり)
    #[test]
    fn normal_003() {
        let tokens = lexer("1 + 10;\n2 - 30;\n14 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(11),
                Value::Number(-28),
                Value::Number(7),
            ]
        );
    }

    // 変数宣言(数値の代入)
    #[test]
    fn normal_004() {
        let tokens = lexer("let x = 1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
            ]
        );
    }

    // 変数宣言(数式の代入)
    #[test]
    fn normal_005() {
        let tokens = lexer("let x = 1 + 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
            ]
        );
    }

    // 変数宣言(関数の代入)
    #[test]
    fn normal_006() {
        let tokens = lexer("let x = abs(-10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
            ]
        );
    }

    // 解析時にletの中身が期待するものか
    #[test]
    fn normal_007() {
        let tokens = lexer("let x = 10;");
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse().unwrap();

        let stmts = stmt.as_slice();

        match &stmts[0] {
            Stmt::Let { name, expr } => {
                assert_eq!(name, "x");
                assert_eq!(expr, &Expr::Value(Value::Number(10)));
            },
            _ => {
                panic!("ここではStmt:Letを期待しています。");
            }
        }
    }

    // print(boolean), println(boolean)のパターン
    #[test]
    fn normal_008() {
        let tokens = lexer("print(true); println(false);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
            ]
        );
    }

    // 変数に設定したパターン
    #[test]
    fn normal_009() {
        let tokens = lexer("let x = true; x; let x = false; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
                Value::Unit,
                Value::Boolean(false),
            ]
        );
    }
}