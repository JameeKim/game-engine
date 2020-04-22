use crate::math::Translation3;
use serde::{Deserialize, Serialize};

/// Component for entities that are located somewhere in the world at the given position
///
/// # Serialization
/// This struct is represented as `[f32; 3]` when being (de)serialized.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(
    from = "position_serde::SerializedPosition",
    into = "position_serde::SerializedPosition"
)]
pub struct Position(Translation3<f32>);

impl Default for Position {
    fn default() -> Self {
        Position(Translation3::new(0.0, 0.0, 0.0))
    }
}

impl Position {
    /// Create a new instance with identity value
    pub fn new() -> Position {
        Position::default()
    }

    /// Create a new instance with identity value
    pub fn zero() -> Position {
        Position::new()
    }

    /// Create a new instance with identity value
    pub fn identity() -> Position {
        Position::new()
    }

    /// Create a new instance with the given coordinate values
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Position {
        Position(Translation3::new(x, y, z))
    }

    /// Create a new instance with the given x coordinate value and the rest or the coordinate values set to zero
    pub fn from_x(x: f32) -> Position {
        Position::from_xyz(x, 0.0, 0.0)
    }

    /// Create a new instance with the given y coordinate value and the rest or the coordinate values set to zero
    pub fn from_y(y: f32) -> Position {
        Position::from_xyz(0.0, y, 0.0)
    }

    /// Create a new instance with the given z coordinate value and the rest or the coordinate values set to zero
    pub fn from_z(z: f32) -> Position {
        Position::from_xyz(0.0, 0.0, z)
    }
}

mod position_conversion {
    use super::Position;
    use crate::math::{Point3, Translation3, Vector3};
    use std::ops::{Deref, DerefMut};

    impl From<Vector3<f32>> for Position {
        fn from(value: Vector3<f32>) -> Self {
            Position(value.into())
        }
    }

    impl From<Position> for Vector3<f32> {
        fn from(value: Position) -> Self {
            value.0.vector
        }
    }

    impl From<Point3<f32>> for Position {
        fn from(value: Point3<f32>) -> Self {
            Position(value.coords.into())
        }
    }

    impl From<Position> for Translation3<f32> {
        fn from(value: Position) -> Self {
            value.0
        }
    }

    impl Deref for Position {
        type Target = Translation3<f32>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Position {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

mod position_math {
    use super::Position;
    use crate::math::{Point3, Translation3, Vector3};
    use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

    impl Mul<Position> for Translation3<f32> {
        type Output = Position;

        fn mul(self, rhs: Position) -> Self::Output {
            rhs + self
        }
    }

    impl Add<Position> for Position {
        type Output = Position;

        fn add(self, rhs: Position) -> Self::Output {
            self + rhs.0
        }
    }

    impl AddAssign<Position> for Position {
        fn add_assign(&mut self, rhs: Position) {
            *self = *self + rhs;
        }
    }

    impl Add<Translation3<f32>> for Position {
        type Output = Position;

        fn add(self, rhs: Translation3<f32>) -> Self::Output {
            self + rhs.vector
        }
    }

    impl AddAssign<Translation3<f32>> for Position {
        fn add_assign(&mut self, rhs: Translation3<f32>) {
            *self = *self + rhs;
        }
    }

    impl Add<Vector3<f32>> for Position {
        type Output = Position;

        fn add(self, rhs: Vector3<f32>) -> Self::Output {
            Position((self.0.vector + rhs).into())
        }
    }

    impl AddAssign<Vector3<f32>> for Position {
        fn add_assign(&mut self, rhs: Vector3<f32>) {
            *self = *self + rhs;
        }
    }

    impl Add<Point3<f32>> for Position {
        type Output = Position;

        fn add(self, rhs: Point3<f32>) -> Self::Output {
            self + rhs.coords
        }
    }

    impl AddAssign<Point3<f32>> for Position {
        fn add_assign(&mut self, rhs: Point3<f32>) {
            *self = *self + rhs;
        }
    }

    impl Add<[f32; 3]> for Position {
        type Output = Position;

        fn add(self, [x, y, z]: [f32; 3]) -> Self::Output {
            self + Vector3::new(x, y, z)
        }
    }

    impl AddAssign<[f32; 3]> for Position {
        fn add_assign(&mut self, rhs: [f32; 3]) {
            *self = *self + rhs;
        }
    }

    impl Sub<Position> for Position {
        type Output = Position;

        fn sub(self, rhs: Position) -> Self::Output {
            self - rhs.0
        }
    }

    impl SubAssign<Position> for Position {
        fn sub_assign(&mut self, rhs: Position) {
            *self = *self - rhs;
        }
    }

    impl Sub<Translation3<f32>> for Position {
        type Output = Position;

        fn sub(self, rhs: Translation3<f32>) -> Self::Output {
            self - rhs.vector
        }
    }

    impl SubAssign<Translation3<f32>> for Position {
        fn sub_assign(&mut self, rhs: Translation3<f32>) {
            *self = *self - rhs;
        }
    }

    impl Sub<Vector3<f32>> for Position {
        type Output = Position;

        fn sub(self, rhs: Vector3<f32>) -> Self::Output {
            Position((self.0.vector - rhs).into())
        }
    }

    impl SubAssign<Vector3<f32>> for Position {
        fn sub_assign(&mut self, rhs: Vector3<f32>) {
            *self = *self - rhs;
        }
    }

    impl Sub<Point3<f32>> for Position {
        type Output = Position;

        fn sub(self, rhs: Point3<f32>) -> Self::Output {
            self - rhs.coords
        }
    }

    impl SubAssign<Point3<f32>> for Position {
        fn sub_assign(&mut self, rhs: Point3<f32>) {
            *self = *self - rhs;
        }
    }

    impl Sub<[f32; 3]> for Position {
        type Output = Position;

        fn sub(self, [x, y, z]: [f32; 3]) -> Self::Output {
            self - Vector3::new(x, y, z)
        }
    }

    impl SubAssign<[f32; 3]> for Position {
        fn sub_assign(&mut self, rhs: [f32; 3]) {
            *self = *self - rhs;
        }
    }
}

mod position_serde {
    use super::*;

    #[allow(missing_copy_implementations, missing_debug_implementations)]
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct SerializedPosition([f32; 3]);

    impl From<SerializedPosition> for Position {
        fn from(SerializedPosition([x, y, z]): SerializedPosition) -> Self {
            Position(Translation3::new(x, y, z))
        }
    }

    impl From<Position> for SerializedPosition {
        fn from(Position(vec): Position) -> Self {
            SerializedPosition([vec.x, vec.y, vec.z])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Position;
    use std::error::Error;

    #[test]
    fn position_serialize() -> Result<(), Box<dyn Error>> {
        let position = Position::zero();
        assert_eq!(ron::ser::to_string(&position)?, "(0,0,0,)");
        assert_eq!(serde_json::to_string(&position)?, "[0.0,0.0,0.0]");
        Ok(())
    }

    #[test]
    fn position_deserialize() -> Result<(), Box<dyn Error>> {
        let real_position = Position::zero();
        let ron_position: Position = ron::de::from_str("(0.0, 0.0, 0.0)")?;
        assert_eq!(ron_position, real_position);
        let json_position: Position = serde_json::from_str("[0, 0, 0]")?;
        assert_eq!(json_position, real_position);
        Ok(())
    }
}
