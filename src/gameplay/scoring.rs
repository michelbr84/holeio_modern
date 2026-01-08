//! Scoring, leaderboard, and ranking

use crate::gameplay::hole::Hole;

/// Leaderboard entry
#[derive(Clone, Debug)]
pub struct LeaderboardEntry {
    pub id: u32,
    pub name: String,
    pub size: f32,
    pub score: i32,
    pub eliminations: i32,
    pub is_player: bool,
    pub rank_change: i32, // +1 moved up, -1 moved down, 0 no change
}

/// Leaderboard system
pub struct Leaderboard {
    entries: Vec<LeaderboardEntry>,
    previous_ranks: std::collections::HashMap<u32, usize>,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            previous_ranks: std::collections::HashMap::new(),
        }
    }

    /// Update leaderboard from holes
    pub fn update(&mut self, holes: &[Hole]) {
        // Save previous ranks
        for (i, entry) in self.entries.iter().enumerate() {
            self.previous_ranks.insert(entry.id, i);
        }

        // Rebuild entries
        self.entries.clear();
        for hole in holes {
            if hole.is_alive {
                self.entries.push(LeaderboardEntry {
                    id: hole.id,
                    name: hole.name.clone(),
                    size: hole.radius,
                    score: hole.score,
                    eliminations: hole.eliminations,
                    is_player: hole.is_player,
                    rank_change: 0,
                });
            }
        }

        // Sort by size (descending)
        self.entries.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

        // Calculate rank changes
        for (new_rank, entry) in self.entries.iter_mut().enumerate() {
            if let Some(&old_rank) = self.previous_ranks.get(&entry.id) {
                if new_rank < old_rank {
                    entry.rank_change = 1; // Moved up
                } else if new_rank > old_rank {
                    entry.rank_change = -1; // Moved down
                }
            }
        }
    }

    /// Get top N entries
    pub fn top(&self, n: usize) -> &[LeaderboardEntry] {
        let end = n.min(self.entries.len());
        &self.entries[..end]
    }

    /// Get player's rank (1-indexed)
    pub fn get_player_rank(&self) -> Option<usize> {
        self.entries.iter().position(|e| e.is_player).map(|i| i + 1)
    }

    /// Get player's entry
    pub fn get_player_entry(&self) -> Option<&LeaderboardEntry> {
        self.entries.iter().find(|e| e.is_player)
    }

    /// Get winner (first entry)
    pub fn get_winner(&self) -> Option<&LeaderboardEntry> {
        self.entries.first()
    }

    /// Get total number of alive holes
    pub fn alive_count(&self) -> usize {
        self.entries.len()
    }
}

/// Calculate XP from a game
pub fn calculate_xp(
    time_alive: f32,
    objects_consumed: i32,
    eliminations: i32,
    final_rank: usize,
    total_players: usize,
) -> i32 {
    let time_xp = (time_alive / 10.0) as i32;
    let object_xp = objects_consumed * 2;
    let elimination_xp = eliminations * 50;
    
    // Rank bonus (winner gets most)
    let rank_xp = if final_rank == 1 {
        100
    } else if final_rank <= 3 {
        50
    } else {
        10
    };
    
    time_xp + object_xp + elimination_xp + rank_xp
}

/// Medal thresholds for Solo mode
pub fn get_medal_for_percentage(percentage: f32) -> &'static str {
    if percentage >= 100.0 {
        "🏆 Perfect!"
    } else if percentage >= 90.0 {
        "🥇 Gold"
    } else if percentage >= 75.0 {
        "🥈 Silver"
    } else if percentage >= 50.0 {
        "🥉 Bronze"
    } else {
        "Keep trying!"
    }
}
