use super::matrix4_wrapper::SerializedMatrix4;
use crate::math::Matrix4;
use serde::{Deserialize, Serialize};

/// Transform of this entity relative to the world origin
///
/// The value of this component is calculated by either [`WorldTransformUpdate`] system or
/// [`HierarchicalTransformUpdate`] system depending on whether the entity is at the root of the hierarchy or not. Thus,
/// the end use should not trust its value before both of those systems are run. All changes to this component will be
/// overwritten by the systems.
///
/// [`WorldTransformUpdate`]: ../system/fn.build_world_transform_update_system.html
/// [`HierarchicalTransformUpdate`]: ../system/fn.build_hierarchical_transform_update_system.html
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "SerializedMatrix4", into = "SerializedMatrix4")]
pub struct WorldTransform(Matrix4<f32>);

impl Default for WorldTransform {
    fn default() -> Self {
        WorldTransform(Matrix4::identity())
    }
}

impl WorldTransform {
    /// Create a new instance with identity values
    pub fn new() -> WorldTransform {
        WorldTransform::default()
    }

    /// Create a new instance with identity values
    pub fn identity() -> WorldTransform {
        WorldTransform::new()
    }

    /// Get an immutable reference to the matrix that is stored inside
    pub fn matrix(&self) -> &Matrix4<f32> {
        &self.0
    }
}

mod world_transform_conversion {
    use super::WorldTransform;
    use crate::math::Matrix4;
    use std::ops::{Deref, DerefMut};

    impl From<Matrix4<f32>> for WorldTransform {
        fn from(matrix: Matrix4<f32>) -> Self {
            WorldTransform(matrix)
        }
    }

    impl From<WorldTransform> for Matrix4<f32> {
        fn from(value: WorldTransform) -> Self {
            value.0
        }
    }

    impl Deref for WorldTransform {
        type Target = Matrix4<f32>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for WorldTransform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
