/// Minimum speed for the sun cycle.
///
/// This constant defines the lowest allowable speed for the sun cycle,
/// ensuring that the cycle does not progress too slowly.
pub const SUN_CYCLE_SPEED_MIN: f32 = 0.05;

/// Incremental change in speed for the sun cycle.
///
/// This constant defines the amount by which the speed of the sun cycle
/// can be increased or decreased. It is used for adjusting the cycle speed
/// based on user input or other factors.
pub const SUN_CYCLE_SPEED_DELTA: f32 = 0.01;

/// Maximum speed for the sun cycle.
///
/// This constant defines the highest allowable speed for the sun cycle,
/// ensuring that the cycle does not progress too quickly.
pub const SUN_CYCLE_SPEED_MAX: f32 = 1.0;
