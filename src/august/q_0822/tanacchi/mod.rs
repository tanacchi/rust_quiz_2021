use anyhow::{ensure, Result};
use crate::utils::tanacchi::parser::parse_from_iter;
use crate::utils::tanacchi::error::Error as MyError;

#[allow(dead_code)]
fn main(input: &str) -> Result<isize> {
    let mut input = input.split_whitespace();
    let begin: isize = parse_from_iter(&mut input)?;
    let end: isize = parse_from_iter(&mut input)?;
    ensure!(
        begin < end,
        MyError::InvalidInputError("The second number must be greater than the first one.")
    );
    Ok((begin..=end).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::num::ParseIntError;
    use crate::utils::tanacchi::error::Error as MyError;

    #[rstest]
    #[case("0 10", 55)]
    #[case("-10 10", 0)]
    #[case("-10 0", -55)]
    fn should_return_sum_of_range(#[case] input: &str, #[case] expected: isize) {
        let result = main(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case("10")]
    #[case("-1")]
    #[case("-10")]
    fn raise_when_number_of_inputs_is_not_enough(#[case] input: &str) {
        let result = main(&input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_error_match!(err, MyError, MyError::LackOfInputElemsError);
    }

    #[rstest]
    #[case("10 ahi")]
    #[case("0 3.14")]
    #[case("poyo 100")]
    #[case("23.4 50")]
    fn raise_when_input_cannot_be_parsed_to_int(#[case] input: &str) {
        let result = main(&input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.downcast_ref::<ParseIntError>().is_some());
    }

    #[rstest]
    #[case("10 0")]
    #[case("22 12")]
    #[case("3 -3")]
    fn raise_when_the_second_number_is_greater_than_the_first_one(#[case] input: &str) {
        let result = main(&input);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_error_match!(
            err,
            MyError,
            MyError::InvalidInputError(
                "The second number must be greater than the first one."
            )
        );
    }
}
