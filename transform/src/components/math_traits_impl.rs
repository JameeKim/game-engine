use super::{ParentTransform, Position, Rotation, WorldTransform};
use crate::math::Matrix4;
use std::ops::Mul;

impl Mul<Position> for Rotation {
    type Output = Matrix4<f32>;

    fn mul(self, rhs: Position) -> Self::Output {
        self.to_homogeneous().append_translation(&rhs.vector)
    }
}

impl Mul<ParentTransform> for WorldTransform {
    type Output = WorldTransform;

    fn mul(self, rhs: ParentTransform) -> Self::Output {
        (self.matrix() * rhs.matrix()).into()
    }
}

impl Mul<ParentTransform> for ParentTransform {
    type Output = ParentTransform;

    fn mul(self, rhs: ParentTransform) -> Self::Output {
        ParentTransform::from(self.matrix() * rhs.matrix())
    }
}

impl Mul<WorldTransform> for WorldTransform {
    type Output = WorldTransform;

    fn mul(self, rhs: WorldTransform) -> Self::Output {
        WorldTransform::from(self.matrix() * rhs.matrix())
    }
}

macro_rules! impl_froms_for_transforms {
    ($( $transform:ident ),* $(,)?) => {
        $(
            impl From<Position> for $transform {
                fn from(pos: Position) -> Self {
                    $transform::from(pos.to_homogeneous())
                }
            }

            impl From<Rotation> for $transform {
                fn from(rot: Rotation) -> Self {
                    $transform::from(rot.to_homogeneous())
                }
            }
        )*
    };
}

impl_froms_for_transforms!(ParentTransform, WorldTransform);
