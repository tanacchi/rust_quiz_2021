#![allow(dead_code)]

use anyhow::Result;
use std::io::{Read, Write};

fn main<R: Read, W: Write>(src: &mut R, dst: &mut W) -> Result<()> {
    let mut buf = String::new();
    src.read_to_string(&mut buf)?;

    let num = buf.parse::<isize>()?;

    writeln!(dst, "{}", (num % 2).abs())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("42", "0\n")]
    #[case("-8", "0\n")]
    #[case("0", "0\n")]
    fn 正_偶数の際に0が出力される(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[rstest]
    #[case("3", "1\n")]
    #[case("-5", "1\n")]
    fn 正_奇数の際に1が出力される(#[case] input: &str, #[case] expected: &str) {
        let expected = expected.as_bytes();

        let mut stdin_mock = input.as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_ok());
        assert_eq!(stdout_mock, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn 誤_パースに失敗した際はParseIntErrorを返す() {
        use matches::assert_matches;
        use std::num::ParseIntError;

        let mut stdin_mock = "-aaa".as_bytes();
        let mut stdout_mock = vec![];

        let result = main(&mut stdin_mock, &mut stdout_mock);

        assert!(result.is_err());
        let error = result.unwrap_err();

        // errorの種類を検証
        assert_matches!(error.root_cause().downcast_ref::<ParseIntError>(), Some(_));
    }
}
