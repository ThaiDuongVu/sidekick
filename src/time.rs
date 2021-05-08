use crate::app::App;

pub struct Time {
    pub time_scale: f32,
    pub target_frame_rate: u32,
    frame_time: f32,
    previous_time: f32,
    current_time: f32,
}

impl Time {
    // Default constructor to initialize Time
    pub fn new() -> Self {
        return Self {
            frame_time: 0.0,
            time_scale: 1.0,

            previous_time: 0.0,
            current_time: 0.0,

            target_frame_rate: 60,
        };
    }
    // Return current frame time
    pub fn frame_time(&self) -> f32 {
        return self.frame_time;
    }
    // Return frame time at 60fps
    pub fn fixed_frame_time(&self) -> f32 {
        return 1.0 / 60.0;
    }
    // Return current frame rate
    pub fn frame_rate(&self) -> u32 {
        return (1.0 / self.frame_time) as u32;
    }

    pub fn update(&mut self, app: App) {
        
    }
}
