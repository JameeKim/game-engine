use super::{ParentTransform, WorldTransform};
use crate::math::Matrix4;
use serde::{Deserialize, Serialize};

#[allow(missing_copy_implementations, missing_debug_implementations)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct SerializedMatrix4([f32; 16]);

impl From<[f32; 16]> for SerializedMatrix4 {
    fn from(slice: [f32; 16]) -> Self {
        SerializedMatrix4(slice)
    }
}

impl From<SerializedMatrix4> for WorldTransform {
    fn from(value: SerializedMatrix4) -> Self {
        matrix4_from_slice(&value.0).into()
    }
}

impl From<WorldTransform> for SerializedMatrix4 {
    fn from(transform: WorldTransform) -> Self {
        slice_from_matrix4(&transform).into()
    }
}

impl From<SerializedMatrix4> for ParentTransform {
    fn from(value: SerializedMatrix4) -> Self {
        matrix4_from_slice(&value.0).into()
    }
}

impl From<ParentTransform> for SerializedMatrix4 {
    fn from(transform: ParentTransform) -> Self {
        slice_from_matrix4(&transform).into()
    }
}

fn matrix4_from_slice(slice: &[f32; 16]) -> Matrix4<f32> {
    Matrix4::from_row_slice(slice)
}

#[rustfmt::skip]
fn slice_from_matrix4(m: &Matrix4<f32>) -> [f32; 16] {
    [
        m.m11, m.m12, m.m13, m.m14,
        m.m21, m.m22, m.m23, m.m24,
        m.m31, m.m32, m.m33, m.m34,
        m.m41, m.m42, m.m43, m.m44,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn matrix4_serialize() -> Result<(), Box<dyn Error>> {
        let world_transform = WorldTransform::identity();
        assert_eq!(
            ron::ser::to_string(&world_transform)?,
            "(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,)"
        );
        Ok(())
    }

    #[test]
    fn matrix4_deserialize() -> Result<(), Box<dyn Error>> {
        let real_transform = WorldTransform::identity();
        let ron_str = r#"(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )"#;
        assert_eq!(
            ron::de::from_str::<WorldTransform>(ron_str)?,
            real_transform
        );
        Ok(())
    }
}
