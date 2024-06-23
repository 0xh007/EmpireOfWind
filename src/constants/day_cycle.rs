/// Minimum speed for the day/night cycle.
///
/// This constant defines the lowest allowable speed for the day/night cycle,
/// ensuring that the cycle does not progress too slowly.
pub const DAY_CYCLE_SPEED_MIN: f32 = 0.05;

/// Incremental change in speed for the day/night cycle.
///
/// This constant defines the amount by which the speed of the day/night cycle
/// can be increased or decreased. It is used for adjusting the cycle speed
/// based on user input or other factors.
pub const DAY_CYCLE_SPEED_DELTA: f32 = 0.01;

/// Maximum speed for the day/night cycle.
///
/// This constant defines the highest allowable speed for the day/night cycle,
/// ensuring that the cycle does not progress too quickly.
pub const DAY_CYCLE_SPEED_MAX: f32 = 1.0;
