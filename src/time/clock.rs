//! Game timer with pause support

/// Game clock with countdown
pub struct GameClock {
    /// Total duration in seconds
    pub duration: f32,
    /// Time remaining in seconds
    pub remaining: f32,
    /// Is the clock running?
    pub running: bool,
    /// Time elapsed since game start
    pub elapsed: f32,
}

impl GameClock {
    /// Create a new clock with the given duration
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            remaining: duration,
            running: false,
            elapsed: 0.0,
        }
    }

    /// Start the clock
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Pause the clock
    pub fn pause(&mut self) {
        self.running = false;
    }

    /// Resume the clock
    pub fn resume(&mut self) {
        self.running = true;
    }

    /// Reset the clock
    pub fn reset(&mut self) {
        self.remaining = self.duration;
        self.elapsed = 0.0;
        self.running = false;
    }

    /// Update the clock, returns true if time just ran out
    pub fn update(&mut self, dt: f32) -> bool {
        if !self.running {
            return false;
        }

        self.elapsed += dt;
        let was_positive = self.remaining > 0.0;
        self.remaining -= dt;
        
        if self.remaining <= 0.0 {
            self.remaining = 0.0;
            self.running = false;
            return was_positive; // Time just ran out
        }
        
        false
    }

    /// Check if time has run out
    pub fn is_finished(&self) -> bool {
        self.remaining <= 0.0
    }

    /// Get remaining minutes
    pub fn minutes(&self) -> i32 {
        (self.remaining / 60.0).floor() as i32
    }

    /// Get remaining seconds (0-59)
    pub fn seconds(&self) -> i32 {
        (self.remaining % 60.0).floor() as i32
    }

    /// Get formatted time string
    pub fn formatted(&self) -> String {
        format!("{:02}:{:02}", self.minutes(), self.seconds())
    }

    /// Get progress (0.0 to 1.0, where 1.0 means no time left)
    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            0.0
        } else {
            1.0 - (self.remaining / self.duration)
        }
    }
}

impl Default for GameClock {
    fn default() -> Self {
        Self::new(120.0) // 2 minutes default
    }
}
