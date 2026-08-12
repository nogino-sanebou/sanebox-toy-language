
#[cfg(test)]
mod tests {
    use sanebox::*;

    #[test]
    fn normal_001() {
        let tokens = lexer("println(12345);");
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

    #[test]
    fn normal_002() {
        let tokens = lexer("println(3 + 2);");
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

    #[test]
    fn normal_003() {
        let tokens = lexer("println(1 + 2 + 5);");
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

    #[test]
    fn normal_004() {
        let tokens = lexer("println(3 + 12 + 7 + 10);");
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

    #[test]
    fn normal_005() {
        let tokens = lexer("println(10 - 7);");
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

    #[test]
    fn normal_006() {
        let tokens = lexer("println(10 - 7 + 2);");
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

    #[test]
    fn normal_007() {
        let tokens = lexer("println(10 - 7 + 2 - 4);");
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

    #[test]
    fn normal_008() {
        let tokens = lexer("println(5 - 7 - 4);");
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

    #[test]
    fn normal_009() {
        let tokens = lexer("println((1 + 2) + 3);");
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

    #[test]
    fn normal_010() {
        let tokens = lexer("println(10 - (3 + 2));");
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

    #[test]
    fn normal_011() {
        let tokens = lexer("println((10 - 3) - 2);");
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

    #[test]
    fn normal_012() {
        let tokens = lexer("println(5 + (10 - 3) - 2);");
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

    #[test]
    fn normal_013() {
        let tokens = lexer("println(((1 + 2) - (3 + 4)) + 5);");
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

    #[test]
    fn normal_014() {
        let tokens = lexer("println((1 + 2 - 3 + 4) + 5);");
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

    #[test]
    fn normal_015() {
        let tokens = lexer("println(1 + (2 - 3 + 4 + 5));");
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

    #[test]
    fn normal_016() {
        let tokens = lexer("print(12 * 10);");
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

    #[test]
    fn normal_017() {
        let tokens = lexer("println((5 - 10) / (1 + 1));");
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

    #[test]
    fn normal_018() {
        let tokens = lexer("println(-20 + 5);");
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

    #[test]
    fn normal_019() {
        let tokens = lexer("abs(-10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(10),
            ]
        );
    }

    #[test]
    fn normal_020() {
        let tokens = lexer("abs(-10) + abs(-20) + abs(30);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(60),
            ]
        );
    }

    #[test]
    fn normal_021() {
        let tokens = lexer("abs(-abs(-20));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(20),
            ]
        );
    }
}