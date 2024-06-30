use std::time::Duration;

use bevy::prelude::{Resource, Timer, TimerMode};
use bevy::time::Stopwatch;

use crate::sun::consts::{SUN_CYCLE_SPEED_MAX, SUN_CYCLE_SPEED_MIN};

/// A resource for managing the day/night cycle in the game.
///
/// The `SunCycleTimer` resource handles the timing and speed of the day/night cycle. It uses a `Timer`
/// to determine when to update the atmospheric_lighting and a `Stopwatch` to keep track of the elapsed time,
/// scaled by the `speed` factor. This allows for incremental updates to the atmospheric_lighting, which is
/// more efficient than updating it every frame.
///
/// # Fields
/// - `update`: A `Timer` that triggers updates at a specified interval.
/// - `time`: A `Stopwatch` that tracks the elapsed time for the cycle.
/// - `speed`: A `f32` that controls the speed of the day/night cycle.
///
/// # Methods
/// - `new(duration: Duration, speed: f32) -> Self`:
///   Creates a new `SunCycleTimer` with the given update interval and speed.
/// - `tick(&mut self, delta: Duration)`:
///   Advances the timer and stopwatch by the given delta time.
/// - `paused(&self) -> bool`:
///   Returns whether the stopwatch is paused.
/// - `toggle_pause(&mut self)`:
///   Toggles the pause state of the stopwatch.
/// - `time(&self) -> f32`:
///   Returns the elapsed time in seconds, scaled by the speed factor.
/// - `update(&self) -> bool`:
///   Returns whether the timer has finished its current cycle.
/// - `update_speed(&mut self, delta: f32)`:
///   Adjusts the speed of the day/night cycle, clamping it between `SUN_CYCLE_SPEED_MIN` and
///   `SUN_CYCLE_SPEED_MAX`.
///
#[derive(Resource)]
pub struct SunCycleTimer {
    pub update: Timer,
    pub time: Stopwatch,
    pub speed: f32,
}

impl SunCycleTimer {
    /// Creates a new `SunCycleTimer`.
    ///
    /// # Parameters
    /// - `duration`: The duration for the update timer.
    /// - `speed`: The initial speed of the day/night cycle.
    ///
    /// # Returns
    /// A new `SunCycleTimer` instance.
    pub fn new(duration: Duration, speed: f32) -> Self {
        Self {
            update: Timer::new(duration, TimerMode::Repeating),
            time: Stopwatch::new(),
            speed,
        }
    }

    /// Advances the timer and stopwatch by the given delta time.
    ///
    /// # Parameters
    /// - `delta`: The amount of time to advance by.
    pub fn tick(&mut self, delta: Duration) {
        if !self.paused() {
            self.update.tick(delta);
            self.time.tick(delta.mul_f32(self.speed));
        }
    }

    /// Checks if the stopwatch is paused.
    ///
    /// # Returns
    /// `true` if the stopwatch is paused, `false` otherwise.
    pub fn paused(&self) -> bool {
        self.time.paused()
    }

    /// Toggles the pause state of the stopwatch.
    pub fn toggle_pause(&mut self) {
        if self.time.paused() {
            self.time.unpause();
        } else {
            self.time.pause();
        }
    }

    /// Returns the elapsed time in seconds, scaled by the speed factor.
    ///
    /// # Returns
    /// The elapsed time in seconds.
    pub fn time(&self) -> f32 {
        self.time.elapsed().as_millis() as f32 / 2000.0
    }

    /// Checks if the timer has finished its current cycle.
    ///
    /// # Returns
    /// `true` if the timer has finished, `false` otherwise.
    pub fn update(&self) -> bool {
        self.update.finished()
    }

    /// Adjusts the speed of the day/night cycle.
    ///
    /// # Parameters
    /// - `delta`: The amount to adjust the speed by.
    ///
    /// The speed is clamped between `SUN_CYCLE_SPEED_MIN` and `SUN_CYCLE_SPEED_MAX`.
    pub fn update_speed(&mut self, delta: f32) {
        self.speed += delta;
        if self.speed < SUN_CYCLE_SPEED_MIN {
            self.speed = SUN_CYCLE_SPEED_MIN;
        }
        if self.speed > SUN_CYCLE_SPEED_MAX {
            self.speed = SUN_CYCLE_SPEED_MAX;
        }
    }
}
