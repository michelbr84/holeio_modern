//! Game modes - Classic, Battle, Solo

/// Available game modes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameMode {
    /// 2 minute timed, biggest hole wins
    Classic,
    /// Last hole standing
    Battle,
    /// Solo challenge, consume 100% of city
    Solo,
}

impl GameMode {
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::Classic => "Classic",
            GameMode::Battle => "Battle",
            GameMode::Solo => "Solo",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            GameMode::Classic => "Be the biggest hole when time runs out!",
            GameMode::Battle => "Last hole standing wins!",
            GameMode::Solo => "Consume 100% of the city!",
        }
    }

    pub fn has_timer(&self) -> bool {
        match self {
            GameMode::Classic => true,
            GameMode::Battle => false,
            GameMode::Solo => true,
        }
    }

    pub fn has_bots(&self) -> bool {
        match self {
            GameMode::Classic => true,
            GameMode::Battle => true,
            GameMode::Solo => false,
        }
    }

    pub fn allows_respawn(&self) -> bool {
        match self {
            GameMode::Classic => true,
            GameMode::Battle => false,
            GameMode::Solo => false,
        }
    }

    pub fn round_duration(&self) -> f32 {
        match self {
            GameMode::Classic => 120.0,  // 2 minutes
            GameMode::Battle => 300.0,   // 5 minutes max
            GameMode::Solo => 120.0,     // 2 minutes
        }
    }
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Classic
    }
}

/// Mode-specific rules
pub struct ModeRules {
    pub mode: GameMode,
    pub bot_count: usize,
    pub respawn_time: f32,
    pub safe_zone_shrink: bool,
}

impl ModeRules {
    pub fn new(mode: GameMode) -> Self {
        match mode {
            GameMode::Classic => Self {
                mode,
                bot_count: 5,
                respawn_time: 3.0,
                safe_zone_shrink: false,
            },
            GameMode::Battle => Self {
                mode,
                bot_count: 5,
                respawn_time: 0.0, // No respawn
                safe_zone_shrink: true,
            },
            GameMode::Solo => Self {
                mode,
                bot_count: 0,
                respawn_time: 0.0,
                safe_zone_shrink: false,
            },
        }
    }
}

/// Victory condition result
#[derive(Clone, Debug)]
pub enum VictoryResult {
    None,
    /// Timer ended, show rankings
    TimeUp { winner_name: String, player_rank: usize },
    /// Player won (last standing)
    PlayerWon,
    /// Player was eliminated
    PlayerEliminated { killer_name: String },
    /// Player consumed all objects
    CityConsumed { percentage: f32 },
}

/// Check victory conditions for current mode
pub fn check_victory(
    mode: &ModeRules,
    time_remaining: f32,
    player_alive: bool,
    alive_hole_count: usize,
    city_consumed_percent: f32,
    is_player_winner: bool,
) -> VictoryResult {
    match mode.mode {
        GameMode::Classic => {
            if time_remaining <= 0.0 {
                VictoryResult::TimeUp {
                    winner_name: String::new(), // Will be filled by caller
                    player_rank: 0,
                }
            } else {
                VictoryResult::None
            }
        }
        GameMode::Battle => {
            if !player_alive {
                VictoryResult::PlayerEliminated {
                    killer_name: String::new(),
                }
            } else if alive_hole_count == 1 && is_player_winner {
                VictoryResult::PlayerWon
            } else if time_remaining <= 0.0 {
                VictoryResult::TimeUp {
                    winner_name: String::new(),
                    player_rank: 0,
                }
            } else {
                VictoryResult::None
            }
        }
        GameMode::Solo => {
            if city_consumed_percent >= 100.0 {
                VictoryResult::CityConsumed { percentage: 100.0 }
            } else if time_remaining <= 0.0 {
                VictoryResult::CityConsumed { percentage: city_consumed_percent }
            } else {
                VictoryResult::None
            }
        }
    }
}
