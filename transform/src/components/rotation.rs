pub use self::rotation_serde::SerializedRotation;

use crate::math::{Unit, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Component for entities that have rotating properties in the game world
///
/// # Serialization
/// This struct is represented by [`SerializedRotation`] enum when (de)serialized. When serialized, it will be
/// automatically serialized as [`SerializedRotation::Quaternion`] variant.
///
/// [`SerializedRotation`]: ./enum.SerializedRotation.html
/// [`SerializedRotation::Quaternion`]: ./enum.SerializedRotation.html#variant.Quaternion
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "SerializedRotation", into = "SerializedRotation")]
pub struct Rotation(UnitQuaternion<f32>);

impl Default for Rotation {
    fn default() -> Self {
        Rotation(UnitQuaternion::identity())
    }
}

impl Rotation {
    /// Create a new instance with the identity value
    pub fn new() -> Rotation {
        Rotation::default()
    }

    /// Create a new instance with the identity value
    pub fn identity() -> Rotation {
        Rotation::new()
    }

    /// Create a new instance using the given axis-angle value
    pub fn from_axis_angle(vector: Vector3<f32>) -> Rotation {
        Rotation(UnitQuaternion::new(vector))
    }

    /// Create a new instance using the given axis and angle values
    pub fn from_axis_and_angle(axis: Unit<Vector3<f32>>, angle: f32) -> Rotation {
        Rotation(UnitQuaternion::from_axis_angle(&axis, angle))
    }

    /// Create a new instance using the given raw axis and angle values
    ///
    /// The axis does not have to be normalized.
    pub fn from_raw_axis_and_angle(axis: Vector3<f32>, angle: f32) -> Rotation {
        Rotation::from_axis_and_angle(Unit::new_normalize(axis), angle)
    }

    /// Create a new instance with the given Euler angle values, in the order of roll, pitch, and yaw
    pub fn from_euler(roll: f32, pitch: f32, yaw: f32) -> Rotation {
        Rotation(UnitQuaternion::from_euler_angles(roll, pitch, yaw))
    }
}

mod rotation_conversion {
    use super::{Rotation, UnitQuaternion};
    use std::ops::{Deref, DerefMut};

    impl From<UnitQuaternion<f32>> for Rotation {
        fn from(quaternion: UnitQuaternion<f32>) -> Self {
            Rotation(quaternion)
        }
    }

    impl From<Rotation> for UnitQuaternion<f32> {
        fn from(rotation: Rotation) -> Self {
            rotation.0
        }
    }

    impl Deref for Rotation {
        type Target = UnitQuaternion<f32>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Rotation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

mod rotation_math {
    use super::{Rotation, UnitQuaternion};
    use std::ops::{Mul, MulAssign};

    impl Mul<Rotation> for Rotation {
        type Output = Rotation;

        fn mul(self, rhs: Rotation) -> Self::Output {
            Rotation(self.0 * rhs.0)
        }
    }

    impl MulAssign<Rotation> for Rotation {
        fn mul_assign(&mut self, rhs: Rotation) {
            *self = *self * rhs;
        }
    }

    impl Mul<UnitQuaternion<f32>> for Rotation {
        type Output = Rotation;

        fn mul(self, rhs: UnitQuaternion<f32>) -> Self::Output {
            Rotation(self.0 * rhs)
        }
    }

    impl MulAssign<UnitQuaternion<f32>> for Rotation {
        fn mul_assign(&mut self, rhs: UnitQuaternion<f32>) {
            *self = *self * rhs;
        }
    }

    impl Mul<Rotation> for UnitQuaternion<f32> {
        type Output = Rotation;

        fn mul(self, rhs: Rotation) -> Self::Output {
            Rotation(self * rhs.0)
        }
    }
}

mod rotation_serde {
    use super::Rotation;
    use crate::math::{Quaternion, UnitQuaternion, Vector3};
    use serde::{Deserialize, Serialize};

    /// Serialization format of [`Rotation`] component
    ///
    /// [`Rotation`]: struct.Rotation.html
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    #[derive(Deserialize, Serialize)]
    pub enum SerializedRotation {
        /// Represented by the angle around the axis
        AxisAngle {
            /// The angle around the axis
            angle: f32,
            /// The axis of the rotation; the value needs to be normalized beforehand
            axis: [f32; 3],
        },

        /// Represented as `(w, i, j, k)` format
        Quaternion(f32, f32, f32, f32),

        /// Represented by Euler angles, in the order of `roll`, `pitch`, and `yaw`
        Euler(f32, f32, f32),
    }

    impl From<SerializedRotation> for Rotation {
        fn from(ser: SerializedRotation) -> Self {
            match ser {
                SerializedRotation::AxisAngle {
                    angle,
                    axis: [x, y, z],
                } => Rotation(UnitQuaternion::new(Vector3::new(x, y, z) * angle)),

                SerializedRotation::Quaternion(w, i, j, k) => {
                    Rotation(UnitQuaternion::from_quaternion(Quaternion::new(w, i, j, k)))
                }

                SerializedRotation::Euler(roll, pitch, yaw) => {
                    Rotation(UnitQuaternion::from_euler_angles(roll, pitch, yaw))
                }
            }
        }
    }

    impl From<Rotation> for SerializedRotation {
        fn from(Rotation(quaternion): Rotation) -> Self {
            SerializedRotation::Quaternion(quaternion.w, quaternion.i, quaternion.j, quaternion.k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn rotation_serialize() -> Result<(), Box<dyn Error>> {
        let rotation = Rotation::new();
        assert_eq!(ron::ser::to_string(&rotation)?, "Quaternion(1,0,0,0,)");
        assert_eq!(
            serde_json::to_string(&rotation)?,
            r#"{"Quaternion":[1.0,0.0,0.0,0.0]}"#
        );
        Ok(())
    }

    #[test]
    fn rotation_deserialize() -> Result<(), Box<dyn Error>> {
        let real_rotation = Rotation::new();
        assert_eq!(
            ron::de::from_str::<Rotation>("Euler(0.0, 0.0, 0.0)")?,
            real_rotation
        );
        assert_eq!(
            serde_json::from_str::<Rotation>(r#"{"AxisAngle": {"angle": 0, "axis": [1, 0, 0]}}"#)?,
            real_rotation
        );
        Ok(())
    }
}
