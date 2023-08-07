use crate::{
    app_error::{AppError, AppResult},
    char_result::CharResult,
    core::GuessResult,
    game_state::GameState,
};
use chrono::Duration;
use error_stack::bail;

#[derive(Debug, PartialEq)]
pub enum SessionState {
    InProgress {
        summaries: Vec<String>,
    },
    Won(String),
    Lost(String),
}
impl SessionState {
    fn won() -> Self {
        SessionState::Won("Well done, you've guessed the word!".to_string())
    }

    fn lost(word: &str) -> Self {
        SessionState::Lost(format!(
            "Sorry, better luck next time. The word was '{}'.",
            word
        ))
    }

    pub fn determined_by(
        user_input: &str,
        game_state: &mut GameState,
    ) -> AppResult<Self> {
        let word_length =
            game_state.word_length();

        if user_input.len()
            != word_length
        {
            bail!(
                AppError::InvalidGuessLength(word_length)
            )
        }

        let user_input =
            user_input.to_lowercase();
        let the_word =
            game_state.word();

        let result = if user_input
            == *the_word
        {
            Self::won()
        } else {
            let attempts_left =
                game_state
                    .attempts_left();

            if attempts_left > 1 {
                let attempt: Vec<_> = user_input
                    .chars()
                    .enumerate()
                    .map(|(i, char)| {
                        let result =
                            match game_state.find_by(&char) {
                                Some(set) if set.contains(&i) => {
                                    GuessResult::Correct
                                }
                                Some(_) => {
                                    GuessResult::Present
                                }
                                None => {
                                    GuessResult::Absent
                                }
                        };

                        CharResult::new(char, result)
                    })
                    .collect();

                let mut summaries =
                    if game_state.last_update_older_than(&Duration::minutes(5)) {
                        game_state.describe()
                    } else {
                        vec![]
                    };

                let session_summary =
                    vec![
                        format!("Your guess was '{}'.", user_input),
                        format!("Here's how you did: {}.", CharResult::display(&attempt)),
                        format!("You now have {} attempts left.", attempts_left - 1),
                    ];

                game_state.add_attempt(
                    attempt,
                );

                summaries.extend(
                    session_summary,
                );

                SessionState::InProgress { summaries }
            } else {
                Self::lost(the_word)
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_app_error;
    use pretty_assertions::assert_eq;

    const WORD: &str = "golem";

    const WRONG_ANSWER: &str = "abcde";

    // fn determine_by()
    #[test]
    fn determined_by_should_return_err_when_user_input_length_is_not_matching(
    ) {
        let mut game_state =
            GameState::of(WORD);

        let user_input = "";

        let actual =
            SessionState::determined_by(
                user_input,
                &mut game_state,
            );
        let expected = AppError::InvalidGuessLength(WORD.len());

        assert_app_error!(
            actual, expected
        );
    }

    #[test]
    fn determined_by_should_return_win_when_user_guesses_it_on_first_try(
    ) {
        let mut game_state =
            GameState::of(WORD);
        let session_state =
            SessionState::determined_by(WORD, &mut game_state).unwrap();

        assert_eq!(
            session_state,
            SessionState::won()
        );
    }

    struct TestArgs<'a> {
        last_attempt: usize,
        last_answer: &'a str,
        expected: SessionState,
    }

    fn test_determined_by(
        args: TestArgs,
    ) {
        let mut game_state =
            GameState::of(WORD);

        let attempts = args.last_attempt
            ..=(game_state
                .attempts_left()
                - 1);

        for n in attempts.rev() {
            let session_state =
                SessionState::determined_by(WRONG_ANSWER, &mut game_state).unwrap();

            assert_eq!(
                session_state,
                SessionState::InProgress {
                    summaries: vec![
                        format!("Your guess was '{}'.", WRONG_ANSWER),
                        format!(
                            "Here's how you did: {}.",
                            "['a' => Absent, 'b' => Absent, 'c' => Absent, 'd' => Absent, 'e' => Present]"
                        ),
                        format!("You now have {} attempts left.", n)
                    ]
                }
            );
        }

        let session_state =
            SessionState::determined_by(args.last_answer, &mut game_state).unwrap();

        assert_eq!(
            session_state,
            args.expected
        );
    }

    #[test]
    fn determined_by_should_return_win_after_playing_a_full_game(
    ) {
        test_determined_by(TestArgs {
            last_attempt: 1,
            last_answer: WORD,
            expected: SessionState::won(
            ),
        });
    }

    #[test]
    fn determined_by_should_return_win_after_user_guesses_it_without_playing_a_full_game(
    ) {
        test_determined_by(TestArgs {
            last_attempt: 2,
            last_answer: WORD,
            expected: SessionState::won(
            ),
        });
    }

    #[test]
    fn determined_by_should_return_lose_after_playing_a_full_game(
    ) {
        test_determined_by(TestArgs {
            last_attempt: 1,
            last_answer: WRONG_ANSWER,
            expected:
                SessionState::lost(WORD),
        });
    }
}
