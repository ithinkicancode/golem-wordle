use crate::core::GuessResult;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CharResult {
    char: char,
    result: GuessResult,
}

impl CharResult {
    pub(crate) fn new(
        char: char,
        result: GuessResult,
    ) -> Self {
        Self { char, result }
    }

    pub(crate) fn correct(
        char: char,
    ) -> Self {
        Self::new(
            char,
            GuessResult::Correct,
        )
    }

    pub(crate) fn absent(
        char: char,
    ) -> Self {
        Self::new(
            char,
            GuessResult::Absent,
        )
    }

    pub(crate) fn present(
        char: char,
    ) -> Self {
        Self::new(
            char,
            GuessResult::Present,
        )
    }

    pub(crate) fn display(
        attempt: &[CharResult],
    ) -> String {
        let joined = attempt
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", joined)
    }
}

impl Display for CharResult {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "'{}' => {:?}",
            self.char, self.result
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    // fn display()
    #[test_case(
        vec![
            CharResult::correct('a'),
            CharResult::absent('b'),
            CharResult::present('c'),
        ],
        "['a' => Correct, 'b' => Absent, 'c' => Present]" ;
        "Non-empty Vec<CharResult> should result in a readable string"
    )]
    #[test_case(
        vec![],
        "[]" ;
        "Empty Vec<CharResult> should result as '[]'"
    )]
    fn display_should_transform_char_results_into_formatted_string(
        attempt: Vec<CharResult>,
        expected: &str,
    ) {
        let actual =
            CharResult::display(
                &attempt,
            );
        assert_eq!(actual, expected);
    }
}
