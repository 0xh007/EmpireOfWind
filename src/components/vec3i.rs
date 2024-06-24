/// A simple 3D vector with integer components.
///
/// The `Vec3I` struct represents a 3D vector using integer values for its components.
/// It provides basic functionality for creating and using 3D vectors in integer space.
///
/// # Fields
/// - `x`: The x-coordinate of the vector.
/// - `y`: The y-coordinate of the vector.
/// - `z`: The z-coordinate of the vector.
///
/// # Usage
///
/// ## Example: Creating and Accessing a Vec3I
///
/// ```rust
/// use empire_of_wind::components::Vec3I;
///
/// // Creating a new Vec3I instance
/// let vector = Vec3I::new(1, 2, 3);
///
/// // Accessing the components of Vec3I
/// println!("Vector: {:?}", vector);
/// println!("x: {}, y: {}, z: {}", vector.x, vector.y, vector.z);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3I {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3I {
    /// Creates a new `Vec3I` with the specified components.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate of the vector.
    /// - `y`: The y-coordinate of the vector.
    /// - `z`: The z-coordinate of the vector.
    ///
    /// # Returns
    /// A new `Vec3I` instance with the specified components.
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3I { x, y, z }
    }
}
