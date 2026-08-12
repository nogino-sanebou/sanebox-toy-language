#[cfg(test)]
mod tests {
    use sanebox::*;

    #[test]
    fn normal_001() {
        let tokens = lexer("1 + 3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(4),
            ]
        );
    }

    #[test]
    fn normal_002() {
        let tokens = lexer("3 - 1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(2),
            ]
        );
    }

    #[test]
    fn normal_003() {
        let tokens = lexer("3 + 4 - 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(2),
            ]
        );
    }

    #[test]
    fn normal_004() {
        let tokens = lexer("3 * 3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(9),
            ]
        );
    }

    #[test]
    fn normal_005() {
        let tokens = lexer("120 / 4;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(30),
            ]
        );
    }

    #[test]
    fn normal_006() {
        let tokens = lexer("10 * 20 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(100),
            ]
        );
    }

    #[test]
    fn normal_007() {
        let tokens = lexer("3 + 5 * 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(28),
            ]
        );
    }

    #[test]
    fn normal_008() {
        let tokens = lexer("3 + 5 * 5 + 10 / 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(30),
            ]
        );
    }

    #[test]
    fn normal_009() {
        let tokens = lexer("20 / 5 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(2),
            ]
        );
    }

    #[test]
    fn normal_010() {
        let tokens = lexer("2 * 3 + 4 * 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(26),
            ]
        );
    }

    #[test]
    fn normal_011() {
        let tokens = lexer("20 / (5 * 2);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(2),
            ]
        );
    }

    #[test]
    fn normal_012() {
        let tokens = lexer("(20 / 5) * 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(8),
            ]
        );
    }

    #[test]
    fn normal_013() {
        let tokens = lexer("-10 + 15;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(5),
            ]
        );
    }

    #[test]
    fn normal_014() {
        let tokens = lexer("6 + -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(4),
            ]
        );
    }

    #[test]
    fn normal_015() {
        let tokens = lexer("6 - -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(8),
            ]
        );
    }

    #[test]
    fn normal_016() {
        let tokens = lexer("-102;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-102),
            ]
        );
    }

    #[test]
    fn normal_017() {
        let tokens = lexer("-(1 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-4),
            ]
        );
    }

    #[test]
    fn normal_018() {
        let tokens = lexer("10 * -3;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-30),
            ]
        );
    }

    #[test]
    fn normal_019() {
        let tokens = lexer("-10 * 4;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-40),
            ]
        );
        match r[0] {
            Value::Number(num) => {
                assert_eq!(-40, num);
            },
            _ => {
                unreachable!()
            },
        }
    }

    #[test]
    fn normal_020() {
        let tokens = lexer("10 / -2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-5),
            ]
        );
    }

    #[test]
    fn normal_021() {
        let tokens = lexer("-10 / 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-5),
            ]
        );
    }

    #[test]
    fn normal_022() {
        let tokens = lexer("-(2 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-6),
            ]
        );
    }

    #[test]
    fn normal_023() {
        let tokens = lexer("(-1 + 6) * (2 -- 3) + -(10 / 2);");
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

    #[test]
    fn normal_024() {
        let tokens = lexer("(3 + 2) - (7 + 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-5),
            ]
        );
    }

    #[test]
    fn normal_025() {
        let tokens = lexer("((3 + 2) - (7 + 3)) + 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(15),
            ]
        );
    }

    #[test]
    fn normal_026() {
        let tokens = lexer("200 + ((100 - 20) + (15 + 20));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(315),
            ]
        );
    }

    #[test]
    fn normal_027() {
        let tokens = lexer("((10 + 20) - (30 + 70) - (200 - 150));");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(-120),
            ]
        );
    }

    #[test]
    fn normal_028() {
        let tokens = lexer("(10 + 1) * (3 + 8);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(121),
            ]
        );
    }

    #[test]
    fn normal_029() {
        let tokens = lexer("10 - - -1;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(9),
            ]
        );
    }

    #[test]
    fn normal_030() {
        let tokens = lexer("1 + abs(-10); abs(-10) - 5; 2 * abs(-10) / 5;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Number(11),
                Value::Number(5),
                Value::Number(4),
            ]
        );
    }
}
