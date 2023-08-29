use crate::{
    app_error::{AppError, AppResult},
    char_result::CharResult,
    core::GuessResult,
    game_state::GameState,
};
use chrono::Duration;
use error_stack::bail;
use once_cell::sync::Lazy;

const IDLE_DURATION: i64 = 5;

static IDLE_TIME: Lazy<Duration> =
    Lazy::new(|| {
        Duration::minutes(IDLE_DURATION)
    });

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
                    if game_state.last_update_older_than(
                        &IDLE_TIME
                    ) {
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
    use crate::{
        assert_app_error,
        clock::{
            tests::TestClock, RealClock,
        },
        game_state::GAME_INSTRUCTION,
    };
    use pretty_assertions::assert_eq;

    const WORD: &str = "golem";

    const WORD_LENGTH: usize =
        WORD.len();

    const WRONG_ANSWER: &str = "abcde";

    const PREVIOUS_MOVE: &str =
        "['a' => Absent, 'b' => Absent, 'c' => Absent, 'd' => Absent, 'e' => Present]";

    // fn determine_by()
    #[test]
    fn determined_by_should_return_err_when_user_input_length_is_not_matching(
    ) {
        let mut game_state =
            GameState::of(
                WORD, &RealClock,
            );

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
            GameState::of(
                WORD, &RealClock,
            );
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
        args: &TestArgs,
    ) {
        let mut game_state =
            GameState::of(
                WORD, &RealClock,
            );

        let attempts = args.last_attempt
            ..=(game_state
                .attempts_left()
                - 1);

        for n in attempts.rev() {
            let session_state =
                SessionState::determined_by(WRONG_ANSWER, &mut game_state).unwrap();

            let summaries = vec![
                format!("Your guess was '{}'.", WRONG_ANSWER),
                format!(
                    "Here's how you did: {}.",
                    PREVIOUS_MOVE
                ),
                format!("You now have {} attempts left.", n)
            ];

            assert_eq!(
                session_state,
                SessionState::InProgress { summaries }
            );
        }

        let session_state =
            SessionState::determined_by(args.last_answer, &mut game_state).unwrap();

        assert_eq!(
            session_state,
            args.expected
        );
    }

    fn test_determined_by_with_test_clock(
        args: &TestArgs,
    ) {
        let year = 2000;
        let month = 1;
        let day = 1;
        let hour = 1;
        let mut minute = 0;

        let clock = TestClock::init(
            year, month, day, hour,
            minute,
        );

        let idle_minutes =
            IDLE_DURATION + 1;

        let idle_time =
            Duration::minutes(
                idle_minutes,
            );

        let mut game_state =
            GameState::of(WORD, &clock);

        let attempts = args.last_attempt
            ..=(game_state
                .attempts_left()
                - 1);

        let mut i = 0;

        for n in attempts.rev() {
            clock.advance(idle_time);

            let session_state =
                SessionState::determined_by(WRONG_ANSWER, &mut game_state).unwrap();

            let mut summaries = vec![
                format!(
                    "Welcome to Golem Wordle! Please describe Golem in a {}-letter word.",
                    WORD_LENGTH
                )
            ];

            if i > 0 {
                summaries.push(
                    format!("Here are your previous {} guesses.", i),
                );
                summaries.extend(vec![
                        PREVIOUS_MOVE
                            .into();
                        i
                    ]);

                minute +=
                    idle_minutes as u32;

                summaries.push(
                    format!(
                        "Last time you played was on {}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00 UTC.",
                        year,
                        month,
                        day,
                        hour,
                        minute
                    )
                );
            } else {
                summaries.push(
                    format!(
                        "You started this game on {}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00 UTC.",
                        year,
                        month,
                        day,
                        hour,
                        minute
                    )
                );
            }

            summaries.extend(vec![
                format!(
                    "You had {} attempts left.",
                    n + 1
                ),
                GAME_INSTRUCTION.into(),
                format!(
                    "Your guess was '{}'.",
                    WRONG_ANSWER
                ),
                format!(
                    "Here's how you did: {}.",
                    PREVIOUS_MOVE
                ),
                format!("You now have {} attempts left.", n)
            ]);

            assert_eq!(
                session_state,
                SessionState::InProgress { summaries }
            );

            i += 1;
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
        let args = TestArgs {
            last_attempt: 1,
            last_answer: WORD,
            expected: SessionState::won(
            ),
        };
        test_determined_by(&args);
        test_determined_by_with_test_clock(&args);
    }

    #[test]
    fn determined_by_should_return_win_after_user_guesses_it_without_playing_a_full_game(
    ) {
        let args = TestArgs {
            last_attempt: 2,
            last_answer: WORD,
            expected: SessionState::won(
            ),
        };
        test_determined_by(&args);
        test_determined_by_with_test_clock(&args);
    }

    #[test]
    fn determined_by_should_return_lose_after_playing_a_full_game(
    ) {
        let args = TestArgs {
            last_attempt: 1,
            last_answer: WRONG_ANSWER,
            expected:
                SessionState::lost(WORD),
        };
        test_determined_by(&args);
        test_determined_by_with_test_clock(&args);
    }
}
