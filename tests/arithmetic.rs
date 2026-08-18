#[cfg(test)]
mod tests {
    use sanebox::*;

    // 加算
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

    // 減算
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

    // 加算・減算混在
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

    // 乗算
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

    // 除算
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

    // 乗算・除算混在
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

    // 加算・乗算混在
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

    // 加算・乗算・除算混在
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

    // 複数除算
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

    // 乗算・加算混在
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

    // 後方に括弧
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

    // 前方に括弧
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

    // マイナスの数値に別の自然数を加算
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

    // 別な自然数にマイナスの数値を加算
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

    // 別な自然数にマイナスの数値を減算
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

    // マイナス単体
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

    // 括弧を付けたマイナス単体
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

    // 別な自然数にマイナスの数値を乗算
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

    // マイナスの数値に別の自然数を乗算
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

    // 別な自然数にマイナスの数値を除算
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

    // マイナスの数値に別の自然数を除算
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

    // 括弧の中身が乗算のマイナス
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

    // 複数のマイナス値を含めた計算
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

    // 括弧同士の計算
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

    // ２重括弧のある計算(前方)
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

    // ２重括弧のある計算(後方)
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

    // ２重括弧のある計算(全てを括弧で括る)
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

    // 括弧のある乗算
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

    // マイナスが複数あった場合
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

    // 関数と組み合わせた計算
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

    // 変数のみを書いた場合
    #[test]
    fn normal_031() {
        let tokens = lexer("let x = 10; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Number(10),
            ]
        );
    }

    // 変数と数値の計算
    #[test]
    fn normal_032() {
        let tokens = lexer("let x = 10; x * 2;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Number(20),
            ]
        );
    }

    // 変数同士の計算
    #[test]
    fn normal_033() {
        let tokens = lexer("let x = 10; let y = 3; x - y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Number(7),
            ]
        );
    }

    // 関数に変数を渡す
    #[test]
    fn normal_034() {
        let tokens = lexer("let x = -35; abs(x);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Number(35),
            ]
        );
    }

    // 定義した変数を別な変数の宣言時に使用する
    #[test]
    fn normal_035() {
        let tokens = lexer("let x = 10; let y = x + 5; y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Number(15),
            ]
        );
    }

    // 変数の再定義
    #[test]
    fn normal_036() {
        let tokens = lexer("let x = 10; let x = 20; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Number(20),
            ]
        );
    }

    // 数値 < 数値 (trueパターン)
    #[test]
    fn normal_037() {
        let tokens = lexer("10 < 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 < 数値 (falseパターン)
    #[test]
    fn normal_038() {
        let tokens = lexer("20 < 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) < (数値) (括弧パターン)
    #[test]
    fn normal_039() {
        let tokens = lexer("(1 + 2) < (3 - 4);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // 関数 < 数値
    #[test]
    fn normal_040() {
        let tokens = lexer("abs(-10) < 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 < 関数
    #[test]
    fn normal_041() {
        let tokens = lexer("10 < abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 < 数値
    #[test]
    fn normal_042() {
        let tokens = lexer("let x = 10; x < 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 < 変数
    #[test]
    fn normal_043() {
        let tokens = lexer("let x = 10; let y = 20; x < y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 数値 > 数値 (trueパターン)
    #[test]
    fn normal_044() {
        let tokens = lexer("20 > 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 > 数値 (falseパターン)
    #[test]
    fn normal_045() {
        let tokens = lexer("10 > 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) > (数値) (括弧パターン)
    #[test]
    fn normal_046() {
        let tokens = lexer("(1 + 2) > (3 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // 関数 > 数値
    #[test]
    fn normal_047() {
        let tokens = lexer("abs(-30) > 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 > 関数
    #[test]
    fn normal_048() {
        let tokens = lexer("30 > abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 > 数値
    #[test]
    fn normal_049() {
        let tokens = lexer("let x = 20; x > 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 > 変数
    #[test]
    fn normal_050() {
        let tokens = lexer("let x = 20; let y = 10; x > y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 数値 == 数値 (trueパターン)
    #[test]
    fn normal_051() {
        let tokens = lexer("10 == 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 == 数値 (falseパターン)
    #[test]
    fn normal_052() {
        let tokens = lexer("10 == 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) == (数値) (括弧パターン)
    #[test]
    fn normal_053() {
        let tokens = lexer("(4 + 5) == (3 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 関数 == 数値
    #[test]
    fn normal_054() {
        let tokens = lexer("abs(-30) == 30;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 == 関数
    #[test]
    fn normal_055() {
        let tokens = lexer("20 == abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 == 数値
    #[test]
    fn normal_056() {
        let tokens = lexer("let x = 20; x == 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 == 変数
    #[test]
    fn normal_057() {
        let tokens = lexer("let x = 20; let y = 20; x == y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }


    // boolean(比較結果)を変数の初期値にする
    #[test]
    fn normal_058() {
        let tokens = lexer("let x = 20 < 30; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 < 数値)」の形
    #[test]
    fn normal_059() {
        let tokens = lexer("(10 < 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 > 数値)」の形
    #[test]
    fn normal_060() {
        let tokens = lexer("(20 > 10);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 == 数値)」の形
    #[test]
    fn normal_061() {
        let tokens = lexer("(20 == 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 <= 数値 (trueパターン)
    #[test]
    fn normal_062() {
        let tokens = lexer("10 <= 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 <= 数値 (falseパターン)
    #[test]
    fn normal_063() {
        let tokens = lexer("20 <= 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) <= (数値) (括弧パターン)
    #[test]
    fn normal_064() {
        let tokens = lexer("(4 + 5) <= (4 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 関数 <= 数値
    #[test]
    fn normal_065() {
        let tokens = lexer("abs(-30) <= 30;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 <= 関数
    #[test]
    fn normal_066() {
        let tokens = lexer("10 <= abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 <= 数値
    #[test]
    fn normal_067() {
        let tokens = lexer("let x = 20; x <= 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 <= 変数
    #[test]
    fn normal_068() {
        let tokens = lexer("let x = 20; let y = 50; x <= y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }


    // boolean(比較結果)を変数の初期値にする
    #[test]
    fn normal_069() {
        let tokens = lexer("let x = 20 <= 30; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 <= 数値)」の形
    #[test]
    fn normal_070() {
        let tokens = lexer("(10 <= 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 >= 数値 (trueパターン)
    #[test]
    fn normal_071() {
        let tokens = lexer("10 >= 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 <= 数値 (falseパターン)
    #[test]
    fn normal_072() {
        let tokens = lexer("10 >= 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) >= (数値) (括弧パターン)
    #[test]
    fn normal_073() {
        let tokens = lexer("(4 + 5) >= (2 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 関数 >= 数値
    #[test]
    fn normal_074() {
        let tokens = lexer("abs(-30) >= 30;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 >= 関数
    #[test]
    fn normal_075() {
        let tokens = lexer("30 >= abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 >= 数値
    #[test]
    fn normal_076() {
        let tokens = lexer("let x = 20; x >= 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 >= 変数
    #[test]
    fn normal_077() {
        let tokens = lexer("let x = 60; let y = 50; x >= y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }


    // boolean(比較結果)を変数の初期値にする
    #[test]
    fn normal_078() {
        let tokens = lexer("let x = 50 >= 30; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 >= 数値)」の形
    #[test]
    fn normal_079() {
        let tokens = lexer("(20 >= 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 != 数値 (trueパターン)
    #[test]
    fn normal_080() {
        let tokens = lexer("20 != 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 != 数値 (falseパターン)
    #[test]
    fn normal_081() {
        let tokens = lexer("10 != 10;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(false),
            ]
        );
    }

    // (数値) != (数値) (括弧パターン)
    #[test]
    fn normal_082() {
        let tokens = lexer("(4 + 5) != (2 * 3);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 関数 != 数値
    #[test]
    fn normal_083() {
        let tokens = lexer("abs(-20) != 30;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 数値 != 関数
    #[test]
    fn normal_084() {
        let tokens = lexer("30 != abs(-20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }

    // 変数 != 数値
    #[test]
    fn normal_085() {
        let tokens = lexer("let x = 30; x != 20;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 変数 != 変数
    #[test]
    fn normal_086() {
        let tokens = lexer("let x = 60; let y = 50; x != y;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }


    // boolean(比較結果)を変数の初期値にする
    #[test]
    fn normal_087() {
        let tokens = lexer("let x = 50 != 30; x;");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Unit,
                Value::Boolean(true),
            ]
        );
    }

    // 括弧で括った場合「(数値 != 数値)」の形
    #[test]
    fn normal_088() {
        let tokens = lexer("(30 != 20);");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        let r = eval_all(expr).unwrap();

        assert_eq!(
            r,
            vec![
                Value::Boolean(true),
            ]
        );
    }
}
