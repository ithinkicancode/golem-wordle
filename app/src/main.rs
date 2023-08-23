use error_stack::ResultExt;
use lib::{
    app_error::{AppError, AppResult},
    core::{pick_word, with_app_state},
    session_state::SessionState,
};
use std::io;

fn main() -> AppResult<()> {
    with_app_state(|state| {
        let game_state = state.new_game_with(pick_word)?;

        println!(
            "Welcome to Golem Wordle! Please describe Golem in a {}-letter word.",
            game_state.word_length()
        );

        loop {
            println!("\nPlease enter your guess: ");

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .change_context(AppError::StdIoRead)?;

            let session_state = SessionState::determined_by(
                &user_input.trim(),
                game_state,
            );
            let session_state = match session_state {
                Ok(s) => s,
                Err(e) => match e.current_context() {
                    AppError::InvalidGuessLength(_) => {
                        eprintln!("*** ERROR: {}", e);
                        continue;
                    }
                    // we propagate other kind of errors
                    _ => return Err(e),
                },
            };

            println!();

            match session_state {
                SessionState::Won(msg)
                | SessionState::Lost(msg) => {
                    state.set_empty();
                    println!("{msg}");
                    break;
                }
                SessionState::InProgress { summaries } => {
                    println!("{}", summaries.join("\n"));
                }
            }
        }

        Ok(())
    })
}
