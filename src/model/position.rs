use crate::data::PIXELS_PER_METER;
use crate::util::ext::TransformExt;
use bevy::math::Vec2;
use bevy::prelude::Quat;
use bevy::prelude::Transform;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub direction: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32, direction: f32) -> Self {
        return Self { x, y, direction };
    }

    pub fn to_transform(&self, z: f32) -> Transform {
        let mut transform = Transform::from_xyz(self.x, self.y, z);
        transform.rotate(self.to_quaternion());
        transform.scale.x = 1.0 / PIXELS_PER_METER;
        transform.scale.y = 1.0 / PIXELS_PER_METER;
        transform.scale.z = 1.0;
        return transform;
    }

    pub fn to_quaternion(&self) -> Quat {
        return Quat::from_rotation_z(self.direction);
    }

    pub fn xy(&self) -> Vec2 {
        return Vec2::new(self.x, self.y);
    }
}

impl From<&Transform> for Position {
    fn from(transform: &Transform) -> Self {
        return Self {
            x: transform.translation.x,
            y: transform.translation.y,
            direction: transform.direction(),
        };
    }
}
