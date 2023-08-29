use crate::{
    app_error::AppResult, clock::Clock,
    game_state::GameState,
};

pub struct AppState<'a>(
    Option<GameState<'a>>,
);
impl<'a> AppState<'a> {
    pub(crate) const fn empty() -> Self
    {
        Self(None)
    }

    pub fn game_state(
        &self,
    ) -> Option<&GameState<'a>> {
        self.0.as_ref()
    }

    pub fn game_state_as_mut(
        &mut self,
    ) -> Option<&mut GameState<'a>>
    {
        self.0.as_mut()
    }

    pub fn set_empty(&mut self) {
        self.0 = None;
    }

    pub fn new_game_with(
        &mut self,
        f: impl FnOnce()
            -> AppResult<&'a str>,
        clock: &'a impl Clock,
    ) -> AppResult<&mut GameState<'a>>
    {
        let word = f()?;

        self.set_empty();

        let result = self
            .0
            .get_or_insert_with(|| {
                GameState::of(
                    word, clock,
                )
            });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::RealClock;
    use pretty_assertions::assert_eq;

    const DUMMY: &str = "dummy";

    // fn set_empty
    #[test]
    fn set_empty_should_set_the_inner_element_to_none(
    ) {
        let dummy_game_state =
            GameState::of(
                DUMMY, &RealClock,
            );

        let mut app_state = AppState(
            Some(dummy_game_state),
        );
        app_state.set_empty();

        assert!(app_state.0.is_none());
    }

    // fn new_game_with
    #[test]
    fn new_game_with_should_produce_a_game_state(
    ) {
        let mut app_state =
            AppState::empty();

        assert!(app_state.0.is_none());

        let game_state = app_state
            .new_game_with(
                || Ok(DUMMY),
                &RealClock,
            )
            .unwrap();

        assert_eq!(
            game_state.word(),
            DUMMY
        );
    }
}
