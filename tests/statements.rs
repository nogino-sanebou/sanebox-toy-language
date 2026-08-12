#[cfg(test)]
mod tests {
    use sanebox::*;

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
}