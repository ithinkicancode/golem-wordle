use bindings::{export, exports::golem::fib::api::*};
use lib::{
    app_error::AppResultExt,
    core::{pick_word, with_app_state},
    session_state::SessionState,
};

fn no_game_in_progress() -> Vec<String> {
    vec![
        "Currently no game in progress. You can start a new game by using the `new-game` command."
            .to_string(),
    ]
}

struct Wordle;

impl Api for Wordle {
    fn new_game() -> GameResult {
        with_app_state(|state| {
            let game_state = state.new_game_with(pick_word).err_as_string()?;

            Ok(game_state.describe())
        })
    }

    fn continue_game(player_guess: String) -> GameResult {
        with_app_state(|state| {
            let messages = if let Some(game_state) = state.game_state_as_mut() {
                let session_state = SessionState::determined_by(&player_guess.trim(), game_state)?;

                match session_state {
                    SessionState::InProgress { summaries } => summaries,
                    SessionState::Won(msg) | SessionState::Lost(msg) => {
                        state.set_empty();

                        vec![msg]
                    }
                }
            } else {
                no_game_in_progress()
            };

            Ok(messages)
        })
        .err_as_string()
    }

    fn game_status() -> GameResult {
        let result = with_app_state(|state| {
            if let Some(game_state) = state.game_state() {
                game_state.describe()
            } else {
                no_game_in_progress()
            }
        });

        Ok(result)
    }
}
export!(Fib);
