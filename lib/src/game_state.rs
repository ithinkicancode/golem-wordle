use crate::{
    char_result::CharResult,
    clock::{Clock, Gmt},
    core::{char_map_from, CharMap},
};
use chrono::Duration;
use std::collections::HashSet;

pub(crate) const GAME_INSTRUCTION: &str =
    "You can continue this game by using the `continue-game` command, or you can start a new game by using the `new-game` command.";

pub struct GameState<'a> {
    word: &'a str,
    word_length: usize,
    last_update: Gmt,
    char_map: CharMap,
    attempts: Vec<Vec<CharResult>>,
    clock: &'a dyn Clock,
}
impl<'a> GameState<'a> {
    pub(crate) fn of(
        word: &'a str,
        clock: &'a impl Clock,
    ) -> Self {
        let char_map =
            char_map_from(word);

        Self {
            word,
            word_length: word.len(),
            char_map,
            attempts: vec![],
            last_update: clock.now(),
            clock,
        }
    }

    pub(crate) fn add_attempt(
        &mut self,
        attempt: Vec<CharResult>,
    ) {
        self.attempts.push(attempt);

        self.last_update =
            self.clock.now();
    }

    pub(crate) fn word(&self) -> &str {
        self.word
    }

    pub fn word_length(&self) -> usize {
        self.word_length
    }

    pub(crate) fn last_update_older_than(
        &self,
        duration: &Duration,
    ) -> bool {
        (self.clock.now()
            - self.last_update)
            > *duration
    }

    pub(crate) fn find_by(
        &self,
        char: &char,
    ) -> Option<&HashSet<usize>> {
        self.char_map.get(char)
    }

    pub(crate) fn attempts_left(
        &self,
    ) -> usize {
        self.word_length
            - self.attempts.len()
    }

    pub fn describe(
        &self,
    ) -> Vec<String> {
        let mut result = vec![format!(
            "Welcome to Golem Wordle! Please describe Golem in a {}-letter word.",
            self.word_length
        )];

        let count = self.attempts.len();

        let attempts = if count > 0 {
            let mut attempts: Vec<_> = self
                .attempts
                .iter()
                .map(|a| CharResult::display(a))
                .collect();

            attempts.insert(0, format!("Here are your previous {} guesses.", count));

            attempts.push(
                format!("Last time you played was on {}.", self.last_update)
            );

            attempts
        } else {
            vec![format!("You started this game on {}.", self.last_update)]
        };

        result.extend(attempts);

        result.push(format!(
            "You had {} attempts left.",
            self.attempts_left()
        ));

        result.push(
            GAME_INSTRUCTION
                .to_string(),
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::RealClock;
    use chrono::{
        Datelike, TimeZone, Utc,
    };
    use maplit::hashset;
    use once_cell::sync::Lazy;
    use pretty_assertions::{
        assert_eq, assert_ne,
    };

    const TEST_DATE_TIME: Lazy<Gmt> =
        Lazy::new(|| {
            Utc.with_ymd_and_hms(
                2312, 12, 18, 19, 23, 0,
            )
            .unwrap()
        });

    fn new_test_game_state(
        word: &str,
    ) -> GameState {
        let mut game_state =
            GameState::of(
                word, &RealClock,
            );

        game_state.last_update =
            *TEST_DATE_TIME;

        game_state
    }

    // fn of(), fn word(), fn word_length(), fn find_by()
    #[test]
    fn game_state_should_be_created_with_expected_state(
    ) {
        let word = "abbc";
        let game_state =
            new_test_game_state(word);

        assert_eq!(
            game_state.word(),
            word
        );
        assert_eq!(
            game_state.char_map,
            char_map_from(word)
        );
        assert!(game_state
            .attempts
            .is_empty());
        assert_eq!(
            game_state.attempts_left(),
            word.len()
        );
        assert_eq!(
            game_state.last_update,
            *TEST_DATE_TIME
        );
        assert_eq!(
            game_state.find_by(&'a'),
            Some(&hashset! {0})
        );
        assert_eq!(
            game_state.find_by(&'b'),
            Some(&hashset! {1, 2})
        );
        assert_eq!(
            game_state.find_by(&'c'),
            Some(&hashset! {3})
        );
    }

    // fn add_attempt()
    #[test]
    fn add_attempt_should_update_game_state(
    ) {
        let word = "abc";
        let mut game_state =
            new_test_game_state(word);

        assert!(game_state
            .attempts
            .is_empty());
        assert_eq!(
            game_state.attempts_left(),
            word.len()
        );
        assert_eq!(
            game_state.last_update,
            *TEST_DATE_TIME
        );

        let attempt = vec![
            CharResult::correct('a'),
            CharResult::absent('x'),
            CharResult::present('b'),
        ];

        game_state.add_attempt(
            attempt.clone(),
        );

        assert_eq!(
            game_state.attempts.len(),
            1
        );
        assert_eq!(
            *game_state.attempts,
            vec![attempt]
        );
        assert_eq!(
            game_state.attempts_left(),
            word.len() - 1
        );
        assert_ne!(
            game_state
                .last_update
                .year(),
            TEST_DATE_TIME.year()
        );
        assert_ne!(
            game_state.last_update,
            *TEST_DATE_TIME
        );
    }

    // fn describe()
    #[test]
    fn describe_should_not_contain_any_previous_guesses_when_attempts_are_empty(
    ) {
        let word = "abc";
        let actual =
            new_test_game_state(word)
                .describe();
        let expected = vec![
            "Welcome to Golem Wordle! Please describe Golem in a 3-letter word.",
            "You started this game on 2312-12-18 19:23:00 UTC.",
            "You had 3 attempts left.",
            GAME_INSTRUCTION,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn describe_should_contain_previous_guesses_when_attempts_are_not_empty(
    ) {
        let word = "abc";
        let mut game_state =
            new_test_game_state(word);

        game_state.attempts.push(vec![
            CharResult::correct('a'),
            CharResult::absent('x'),
            CharResult::present('b'),
        ]);
        game_state.attempts.push(vec![
            CharResult::correct('a'),
            CharResult::absent('y'),
            CharResult::present('b'),
        ]);

        let actual =
            game_state.describe();

        let expected = vec![
            "Welcome to Golem Wordle! Please describe Golem in a 3-letter word.",
            "Here are your previous 2 guesses.",
            "['a' => Correct, 'x' => Absent, 'b' => Present]",
            "['a' => Correct, 'y' => Absent, 'b' => Present]",
            "Last time you played was on 2312-12-18 19:23:00 UTC.",
            "You had 1 attempts left.",
            GAME_INSTRUCTION,
        ];

        assert_eq!(actual, expected);
    }
}
