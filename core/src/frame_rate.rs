//! Things related to frame rate

use crate::ecs::schedule::Schedulable;
use crate::ecs::system::SystemBuilder;
use crate::sized_queue::SizedQueue;
use crate::time::{duration_to_nanoseconds, Time};
use serde::{Deserialize, Serialize};
use std::thread::{sleep, yield_now};
use std::time::{Duration, Instant};

/// Resource to help keep the frame rate
#[derive(Clone, Copy, Debug)]
pub struct FrameRateKeeper {
    /// How to wait for the time to start the next frame
    pub strategy: FrameRateStrategy,
    /// The time this frame started at
    frame_start: Instant,
    /// The desired duration of a single frame
    frame_duration: Duration,
}

impl Default for FrameRateKeeper {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameRateKeeper {
    /// Create a new instance from default values
    pub fn new() -> Self {
        Self::from_config(FrameRateConfig::default())
    }

    /// Create a new instance from the given configuration
    pub fn from_config(config: FrameRateConfig) -> Self {
        let FrameRateConfig { fps, strategy } = config;
        Self {
            strategy,
            frame_start: Instant::now(),
            frame_duration: frame_duration_from_fps(fps),
        }
    }

    /// Set the desired fps to the given value
    pub fn set_fps(&mut self, fps: u32) {
        self.frame_duration = frame_duration_from_fps(fps);
    }

    /// Reset the start time of this frame
    pub fn reset(&mut self) {
        self.frame_start = Instant::now();
    }

    /// Wait until the next frame should start
    pub fn wait_next_frame(&mut self) {
        match self.strategy {
            FrameRateStrategy::AsFastAsPossible => yield_now(),
            FrameRateStrategy::Precise => self.yield_until_next_frame(),
            FrameRateStrategy::EqualOrSlower => self.sleep_until_left(Duration::default()),
            FrameRateStrategy::Rough { until_left } => self.sleep_until_left(until_left),
        }
    }

    fn yield_until_next_frame(&self) {
        yield_now();

        while Instant::now().duration_since(self.frame_start) < self.frame_duration {
            yield_now();
        }
    }

    fn sleep_until_left(&self, until_left: Duration) {
        let sleep_for = self.frame_duration - until_left;

        loop {
            let slept_for = Instant::now() - self.frame_start;
            if slept_for < sleep_for {
                sleep(sleep_for - slept_for);
            } else {
                break;
            }
        }

        self.yield_until_next_frame();
    }
}

impl From<FrameRateConfig> for FrameRateKeeper {
    fn from(config: FrameRateConfig) -> Self {
        Self::from_config(config)
    }
}

/// Configuration for frame rate keeping
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct FrameRateConfig {
    /// Desired frame per second
    pub fps: u32,
    /// How to wait for the time to start the next frame
    pub strategy: FrameRateStrategy,
}

impl Default for FrameRateConfig {
    fn default() -> Self {
        Self {
            fps: 144,
            strategy: FrameRateStrategy::default(),
        }
    }
}

impl FrameRateConfig {
    /// Create a new instance with default values
    pub fn new() -> Self {
        Self::default()
    }
}

/// How to wait for the time to start the next frame
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum FrameRateStrategy {
    /// Ignore the desired fps value and make the frame rate as fast as possible
    AsFastAsPossible,
    /// Keep the frame rate as close as possible to the desired fps value
    ///
    /// This is the default value.
    Precise,
    /// Keep the frame rate equal to or slower than the desired fps value
    ///
    /// This is achieved by telling the system to make this process sleep for at least the frame
    /// duration.
    EqualOrSlower,
    /// Try to roughly keep up to the desired fps value by sleeping until the given value of
    /// duration remains and then doing the same thing as [`Precise`]
    ///
    /// [`Precise`]: #variant.Precise
    Rough {
        /// How much time to leave at most before this frame should end
        until_left: Duration,
    },
}

impl Default for FrameRateStrategy {
    fn default() -> Self {
        FrameRateStrategy::Precise
    }
}

/// Get the duration of a single frame from the given fps value
pub fn frame_duration_from_fps(fps: u32) -> Duration {
    Duration::from_secs(1) / fps
}

/// ECS resource for storing fps values
///
/// This resource is written by [`FpsCount`] system. The actual stored values are the duration of
/// frames in nanoseconds.
///
/// [`FpsCount`]: ./fn.build_fps_count_system.html
#[derive(Clone, Debug)]
pub struct FpsValue {
    past_fps: SizedQueue<u64>,
}

impl Default for FpsValue {
    fn default() -> Self {
        FpsValue::new()
    }
}

impl FpsValue {
    /// Create a new fps value store that keeps the default amount of the last fps values
    ///
    /// The default size is `10`.
    pub fn new() -> Self {
        FpsValue::from_queue_size(10)
    }

    /// Create a new fps value store that keeps the last <size> fps values
    pub fn from_queue_size(size: usize) -> Self {
        FpsValue {
            past_fps: SizedQueue::new(size),
        }
    }

    /// Get the fps value of the last frame
    ///
    /// If there are no stored values, it returns `0.0`.
    pub fn last_fps(&self) -> f32 {
        self.past_fps
            .newest()
            .copied()
            .map(|nanoseconds| {
                if nanoseconds == 0 {
                    0.0
                } else {
                    nanoseconds_to_fps(nanoseconds as f32)
                }
            })
            .unwrap_or_default()
    }

    /// Get the average value of fps of the last some frames
    ///
    /// If there are no stored values, it returns `0.0`.
    pub fn average_fps(&self) -> f32 {
        let sum: u64 = self.past_fps.iter().copied().sum();
        if sum == 0 {
            0.0
        } else {
            nanoseconds_to_fps(sum as f32 / self.past_fps.len() as f32)
        }
    }
}

fn nanoseconds_to_fps(nanoseconds: f32) -> f32 {
    1.0e9 / nanoseconds
}

/// Build a system that measures frame per second value
///
/// The `size` parameter is for determining the size of the queue that stores the last fps values.
/// Thus, it impacts how the average fps value is calculated.
pub fn build_fps_count_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("FpsCount")
        .write_resource::<FpsValue>()
        .read_resource::<Time>()
        .build(|_cmd, _world, resources, _queries| {
            let (fps_value, time) = resources;
            let frame_duration = duration_to_nanoseconds(time.real_delta_duration());
            fps_value.past_fps.push(frame_duration);
        })
}

#[cfg(test)]
mod tests {
    use super::FrameRateKeeper;
    use crate::time::Timer;

    #[test]
    fn frame_rate_keeper() {
        let mut timer = Timer::new();
        let mut keeper = FrameRateKeeper::new();

        timer.start();
        keeper.reset();
        keeper.wait_next_frame();

        let duration = timer.get();
        assert!((duration - keeper.frame_duration).as_nanos() < 100_000);
    }
}
