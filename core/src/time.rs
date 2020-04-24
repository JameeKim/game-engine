//! Measuring and keeping track of time

use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
/// Timer that measures time
pub enum Timer {
    /// The timer is doing nothing
    Idle {
        /// The measured value from previous run; defaults to zero
        measured: Duration,
    },
    /// The timer is running
    Running {
        /// The time the timer started to run
        from: Instant,
        /// The value this timer started with
        start_value: Duration,
    },
}

impl Default for Timer {
    fn default() -> Self {
        Timer::Idle {
            measured: Duration::default(),
        }
    }
}

impl Timer {
    /// Create a new instance of the timer
    pub fn new() -> Timer {
        Timer::default()
    }

    /// Is this timer running?
    pub fn is_running(&self) -> bool {
        if let Timer::Running { .. } = self {
            true
        } else {
            false
        }
    }

    /// Get the measured value
    ///
    /// This returns the measured value from the previous run if the timer is idle. If it is in
    /// running state, the current measured value is given.
    pub fn get(&self) -> Duration {
        match *self {
            Timer::Idle { measured } => measured,
            Timer::Running { from, start_value } => {
                start_value + Instant::now().duration_since(from)
            }
        }
    }

    /// Start the timer, preserving the stored value from the previous run
    pub fn start(&mut self) {
        match *self {
            Timer::Idle { measured } => {
                *self = Timer::Running {
                    from: Instant::now(),
                    start_value: measured,
                };
            }
            _ => {}
        }
    }

    /// Stop the timer if running
    pub fn stop(&mut self) {
        if let Timer::Running { from, start_value } = *self {
            *self = Timer::Idle {
                measured: start_value + Instant::now().duration_since(from),
            };
        }
    }

    /// Reset the timer, discarding all stored values and returning to idle state
    pub fn reset(&mut self) {
        *self = Timer::new();
    }

    /// Start the timer from a fresh value
    pub fn restart(&mut self) {
        *self = Timer::Running {
            from: Instant::now(),
            start_value: Duration::default(),
        };
    }
}

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
        self.delta_duration = duration_from_seconds(self.delta_time);
        self.real_total_duration += self.real_delta_duration;
        self.total_duration += self.delta_duration;
    }

    /// Set the fixed delta time
    pub fn set_fixed_delta_duration(&mut self, duration: Duration) {
        self.real_fixed_delta_time = duration_to_f32(duration);
        self.real_fixed_delta_duration = duration;
        self.fixed_delta_time = self.real_fixed_delta_time * self.scale;
        self.fixed_delta_duration = duration_from_seconds(self.fixed_delta_time)
    }

    /// Set the time scale factor
    pub fn set_scale(&mut self, new_scale: f32) {
        assert!(new_scale >= 0.0);
        assert_ne!(new_scale, std::f32::INFINITY);
        self.scale = new_scale;
    }
}

/// Convert the given [`Duration`] to seconds value in [`f32`] format
pub fn duration_to_f32(duration: Duration) -> f32 {
    duration.as_secs_f32() + duration.subsec_nanos() as f32 / 1.0e9
}

/// Convert the given [`Duration`] to seconds value in [`f64`] format
pub fn duration_to_f64(duration: Duration) -> f64 {
    duration.as_secs_f64() + duration.subsec_nanos() as f64 / 1.0e9
}

/// Convert the given [`Duration`] to nanoseconds value in [`u64`] format
///
/// # Overflow
///
/// The returned value might overflow if the given duration is too long.
pub fn duration_to_nanoseconds(duration: Duration) -> u64 {
    duration.as_nanos() as u64
}

/// Convert the given seconds value in [`f32`] format to [`Duration`]
pub fn duration_from_seconds(seconds: f32) -> Duration {
    Duration::new(seconds as u64, ((seconds % 1.0) * 1.0e9) as u32)
}
