use super::matrix4_wrapper::SerializedMatrix4;
use crate::math::Matrix4;
use serde::{Deserialize, Serialize};

/// Transform of this entity relative to its parent, calculated from any of [`Position`] or [`Rotation`] components this
/// entity has
///
/// This is not in sync with the values from [`Position`] and [`Rotation`] components before [`ParentTransformUpdate`]
/// system is run. Thus, the end users should not trust this value unless that system is run before.
///
/// [`Position`]: ./struct.Position.html
/// [`Rotation`]: ./struct.Rotation.html
/// [`ParentTransformUpdate`]: ../system/fn.build_parent_transform_update_system.html
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "SerializedMatrix4", into = "SerializedMatrix4")]
pub struct ParentTransform(Matrix4<f32>);

impl Default for ParentTransform {
    fn default() -> Self {
        ParentTransform(Matrix4::identity())
    }
}

impl ParentTransform {
    /// Create a new instance with identity values
    pub fn new() -> ParentTransform {
        ParentTransform::default()
    }

    /// Create a new instance with identity values
    pub fn identity() -> ParentTransform {
        ParentTransform::new()
    }

    /// Get an immutable reference to the matrix that is stored inside
    pub fn matrix(&self) -> &Matrix4<f32> {
        &self.0
    }
}

mod parent_transform_conversion {
    use super::ParentTransform;
    use crate::math::Matrix4;
    use std::ops::{Deref, DerefMut};

    impl From<Matrix4<f32>> for ParentTransform {
        fn from(matrix: Matrix4<f32>) -> Self {
            ParentTransform(matrix)
        }
    }

    impl From<ParentTransform> for Matrix4<f32> {
        fn from(value: ParentTransform) -> Self {
            value.0
        }
    }

    impl Deref for ParentTransform {
        type Target = Matrix4<f32>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for ParentTransform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
