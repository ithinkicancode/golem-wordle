use crate::{
    app_error::{
        AppError, AppResult,
        AppResultExt,
    },
    app_state::AppState,
};
use error_stack::{bail, ResultExt};
use once_cell::sync::Lazy;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    str::from_utf8,
};

pub(crate) const WORDS_FILE_PATH: &str =
    "assets/words.txt";

#[allow(clippy::unwrap_used)]
static WORDS: Lazy<Vec<String>> =
    Lazy::new(|| {
        load_words()
            .err_as_string()
            .unwrap()
    });

static WORD_POOL_SIZE: Lazy<usize> =
    Lazy::new(|| WORDS.len());

static mut APP_STATE: AppState =
    AppState::empty();

#[allow(unsafe_code)]
pub fn with_app_state<T>(
    f: impl FnOnce(
        &mut AppState<'static>,
    ) -> T,
) -> T {
    unsafe { f(&mut APP_STATE) }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GuessResult {
    Correct,
    Present,
    Absent,
}

pub(crate) type CharMap =
    HashMap<char, HashSet<usize>>;

pub(crate) fn char_map_from(
    s: &str,
) -> CharMap {
    let mut hashmap: CharMap =
        HashMap::new();

    for (i, c) in s.chars().enumerate()
    {
        hashmap
            .entry(c)
            .and_modify(|set| {
                set.insert(i);
            })
            .or_insert(HashSet::from(
                [i],
            ));
    }

    hashmap
}

fn words_from(
    bytes: &[u8],
) -> AppResult<Vec<String>> {
    let file_content = from_utf8(bytes)
        .change_context(
            AppError::InvalidCharset,
        )?
        .to_lowercase();

    let words: Vec<_> = file_content
        .lines()
        .flat_map(|l| l.split(' '))
        .filter(|s| !{ s.is_empty() })
        .map(|s| s.to_string())
        .collect();

    if words.is_empty() {
        bail!(AppError::NoWords)
    }
    Ok(words)
}

pub(crate) fn load_words(
) -> AppResult<Vec<String>> {
    let bytes: &[u8] = include_bytes!(
        "../../assets/words.txt"
    );

    words_from(bytes)
}

pub(crate) fn random_number(
    upper_bound: usize,
) -> usize {
    rand::thread_rng()
        .gen_range(0..upper_bound)
}

pub fn pick_word<'a>(
) -> AppResult<&'a str> {
    let index =
        random_number(*WORD_POOL_SIZE);

    if let Some(chosen_word) =
        WORDS.get(index)
    {
        // println!(
        //     "This word is chosen: {}",
        //     chosen_word
        // );

        Ok(chosen_word)
    } else {
        bail!(AppError::NoWords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_app_error;
    use error_stack::{report, Report};
    use maplit::{hashmap, hashset};
    use pretty_assertions::assert_eq;
    use proptest::prelude::{
        prop_assert, proptest,
    };
    use test_case::test_case;

    // fn char_map_from()
    #[test_case(
        "Hello",
        hashmap!{
            'H' => hashset!{0},
            'e' => hashset!{1},
            'l' => hashset!{2, 3},
            'o' => hashset!{4}
        } ;
        "a string is turned into a HashMap<char, HashSet<index_position>>"
    )]
    #[test_case(
        "",
        hashmap!{} ;
        "an empty string is turned into an empty HashMap"
    )]
    #[test_case(
        "   ",
        hashmap!{ ' ' => hashset!{0, 1, 2} } ;
        "a string with just spaces is turned into a HashMap of just one entry"
    )]
    fn char_map_from_should_produce_char_map_from_string(
        input: &str,
        expected: CharMap,
    ) {
        let actual =
            char_map_from(input);

        assert_eq!(actual, expected);
    }

    // fn words_from()
    const GOLEM_IS_INVINCIBLE:
        &[&str] =
        &["golem", "is", "invincible"];

    #[test_case(
        "Hello",
        &vec!["hello"] ;
        "single-item vec when the input byte array has one word."
    )]
    #[test_case(
        "golem is invincible",
        GOLEM_IS_INVINCIBLE ;
        "multiple-item vec when the input byte array has multiple words."
    )]
    #[test_case(
        " golem     is \n invincible ",
        GOLEM_IS_INVINCIBLE ;
        "multiple-item vec when the input byte array has multiple words and multiple newlines."
    )]
    #[test_case(
        "    golem   \n    is \n invincible ",
        GOLEM_IS_INVINCIBLE ;
        "multiple-item vec when the input byte array has multiple words and even more multiple newlines."
    )]
    fn words_from_should_produce_word_vec_from_byte_array(
        input: &str,
        expected: &[&str],
    ) {
        let bytes = input.as_bytes();
        let actual =
            words_from(bytes).unwrap();

        assert_eq!(actual, expected)
    }

    fn words_from_base_fail_test(
        expected_error: Report<
            AppError,
        >,
        input: &str,
    ) {
        let bytes = input.as_bytes();

        let actual = words_from(bytes);

        assert_app_error!(
            actual,
            expected_error
        );
    }

    #[test_case(
        "        " ;
        "AppError::NoWords when the input byte array contains no words."
    )]
    #[test_case(
        "    \n     \n  " ;
        "AppError::NoWords when the multiline input byte array contains no words."
    )]
    #[test_case(
        "" ;
        "AppError::NoWords when the input byte array is empty."
    )]
    fn words_from_should_fail_when_input_byte_array_contains_no_words(
        input: &str,
    ) {
        words_from_base_fail_test(
            report!(AppError::NoWords),
            input,
        );
    }

    #[allow(
        invalid_from_utf8_unchecked
    )]
    #[test_case(
        unsafe {
            std::str::from_utf8_unchecked(
                b"cl\x82ippy",
            )
        };
        "AppError::InvalidCharset when file contains non-UTF-8 characters."
    )]
    fn words_from_should_fail_when_input_byte_array_contains_invalid_charset(
        input: &str,
    ) {
        words_from_base_fail_test(
            report!(AppError::InvalidCharset),
            input,
        );
    }

    // fn random_number()
    proptest! {
        #[test]
        fn random_number_should_produce_a_number_within_bounds(
            upper_bound in 1_usize..=100
        ) {
            let result = random_number(upper_bound);

            prop_assert!(result < upper_bound);
        }
    }
}
