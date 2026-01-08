//! Game state management - Menu/Playing/Pause/Results

use crate::gameplay::modes::GameMode;

/// Main game states
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    ModeSelect,
    Playing,
    Pause,
    Results,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Menu
    }
}

/// Complete application state
pub struct AppState {
    pub game_state: GameState,
    pub selected_mode: GameMode,
    pub player_name: String,
    pub menu_selection: usize,
    pub mode_selection: usize,
    pub pause_selection: usize,
    pub results_selection: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            game_state: GameState::Menu,
            selected_mode: GameMode::Classic,
            player_name: "Player".to_string(),
            menu_selection: 0,
            mode_selection: 0,
            pause_selection: 0,
            results_selection: 0,
        }
    }
}

impl AppState {
    pub fn transition_to(&mut self, state: GameState) {
        self.game_state = state;
        // Reset selections on state change
        match state {
            GameState::Menu => self.menu_selection = 0,
            GameState::ModeSelect => self.mode_selection = 0,
            GameState::Pause => self.pause_selection = 0,
            GameState::Results => self.results_selection = 0,
            _ => {}
        }
    }

    pub fn start_game(&mut self, mode: GameMode) {
        self.selected_mode = mode;
        self.game_state = GameState::Playing;
    }
}
