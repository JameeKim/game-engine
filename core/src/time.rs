use std::time::Duration;

/// Manage time in the engine
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    delta_time: f32,
    delta_duration: Duration,
    real_delta_time: f32,
    real_delta_duration: Duration,
    fixed_delta_time: f32,
    fixed_delta_duration: Duration,
    real_fixed_delta_time: f32,
    real_fixed_delta_duration: Duration,
    total_duration: Duration,
    real_total_duration: Duration,
    scale: f32,
}

impl Default for Time {
    fn default() -> Self {
        Time {
            delta_time: 0.0,
            delta_duration: Duration::default(),
            real_delta_time: 0.0,
            real_delta_duration: Duration::default(),
            fixed_delta_time: 0.0,
            fixed_delta_duration: Duration::default(),
            real_fixed_delta_time: 0.0,
            real_fixed_delta_duration: Duration::default(),
            total_duration: Duration::default(),
            real_total_duration: Duration::default(),
            scale: 1.0,
        }
    }
}

impl Time {
    /// Create a new instance with default values
    pub fn new() -> Time {
        Time::default()
    }

    /// Get the scaled delta time in seconds
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Get the scaled delta time in duration
    pub fn delta_duration(&self) -> Duration {
        self.delta_duration
    }

    /// Get the real delta time in seconds
    pub fn real_delta_time(&self) -> f32 {
        self.real_delta_time
    }

    /// Get the real delta time in duration
    pub fn real_delta_duration(&self) -> Duration {
        self.real_delta_duration
    }

    /// Get the scaled fixed delta time in seconds
    pub fn fixed_delta_time(&self) -> f32 {
        self.fixed_delta_time
    }

    /// Get the scaled fixed delta time in duration
    pub fn fixed_delta_duration(&self) -> Duration {
        self.fixed_delta_duration
    }

    /// Get the real fixed delta time in seconds
    pub fn real_fixed_delta_time(&self) -> f32 {
        self.real_fixed_delta_time
    }

    /// Get the real fixed delta time in duration
    pub fn real_fixed_delta_duration(&self) -> Duration {
        self.real_fixed_delta_duration
    }

    /// Get the scaled total time that has passed from the start in seconds
    pub fn total_time(&self) -> f64 {
        duration_to_f64(self.total_duration)
    }

    /// Get the scaled total time that has passed from the start in duration
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// Get the real total time that has passed from the start in seconds
    pub fn real_total_time(&self) -> f64 {
        duration_to_f64(self.real_total_duration)
    }

    /// Get the real total time that has passed from the start in duration
    pub fn real_total_duration(&self) -> Duration {
        self.real_total_duration
    }

    /// Set the delta time
    pub fn set_delta_duration(&mut self, duration: Duration) {
        self.real_delta_time = duration_to_f32(duration);
        self.real_delta_duration = duration;
        self.delta_time = self.real_delta_time * self.scale;
        self.delta_duration = f32_to_duration(self.delta_time);
        self.real_total_duration += self.real_delta_duration;
        self.total_duration += self.delta_duration;
    }

    /// Set the fixed delta time
    pub fn set_fixed_delta_duration(&mut self, duration: Duration) {
        self.real_fixed_delta_time = duration_to_f32(duration);
        self.real_fixed_delta_duration = duration;
        self.fixed_delta_time = self.real_fixed_delta_time * self.scale;
        self.fixed_delta_duration = f32_to_duration(self.fixed_delta_time)
    }

    /// Set the time scale factor
    pub fn set_scale(&mut self, new_scale: f32) {
        assert!(new_scale >= 0.0);
        assert!(new_scale != std::f32::INFINITY);
        self.scale = new_scale;
    }
}

fn duration_to_f32(duration: Duration) -> f32 {
    duration.as_secs_f32() + duration.subsec_nanos() as f32 / 1.0e9
}

fn duration_to_f64(duration: Duration) -> f64 {
    duration.as_secs_f64() + duration.subsec_nanos() as f64 / 1.0e9
}

fn f32_to_duration(seconds: f32) -> Duration {
    Duration::new(seconds as u64, ((seconds % 1.0) * 1.0e9) as u32)
}
