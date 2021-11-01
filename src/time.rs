use std::time::Instant;

/// Game time manager
pub struct Time {
    pub time_scale: f32,
    pub target_frame_rate: u32,
    frame_time: f32,
    previous_time: f32,
    current_time: f32,
    start: Instant,
}

impl Time {
    /// Default constructor to initialize Time
    pub fn new() -> Self {
        return Self {
            time_scale: 1.0,
            target_frame_rate: 60,
            frame_time: 0.0,
            previous_time: 0.0,
            current_time: 0.0,
            start: Instant::now(),
        };
    }
    /// Return current frame time
    pub fn frame_time(&self) -> f32 {
        return self.frame_time;
    }
    /// Return expected frame time at current target frame rate
    pub fn fixed_frame_time(&self) -> f32 {
        return 1.0 / self.target_frame_rate as f32;
    }
    /// Return current frame rate
    pub fn frame_rate(&self) -> u32 {
        return (1.0 / self.frame_time) as u32;
    }

    /// Update frame time, is called every update iteration
    pub fn update(&mut self) {
        // Get current time
        self.current_time = self.start.elapsed().as_secs_f32();
        // Get frame time by substracting previous time from current frame time
        self.frame_time = self.current_time - self.previous_time;
        // Set previous time for next frame update
        self.previous_time = self.current_time;

        // Busy wait for frame to pass before next update
        while self.start.elapsed().as_secs_f32()
            < self.current_time + (1. / self.target_frame_rate as f32)
        {}
        self.current_time += 1. / self.target_frame_rate as f32;
    }
}
