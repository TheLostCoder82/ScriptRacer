//! Mathematics and transform module

pub use glam::*;

/// Transform component for position, rotation, and scale
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    /// Creates a transform from a position
    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    /// Converts the transform to a 4x4 matrix
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Returns the forward direction (negative Z)
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Returns the right direction (positive X)
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Returns the up direction (positive Y)
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// Rotates the transform to look at a target position
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let view_matrix = Mat4::look_at_rh(self.position, target, up);
        self.rotation = Quat::from_mat4(&view_matrix.inverse());
    }
}

/// Marker trait for components
pub trait Component: Clone + Send + Sync + 'static + std::fmt::Debug {}

impl Component for Transform {}

/// Math utility functions
pub mod math_utils {
    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// Clamps a value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Converts degrees to radians
    pub fn deg2rad(deg: f32) -> f32 {
        deg.to_radians()
    }

    /// Converts radians to degrees
    pub fn rad2deg(rad: f32) -> f32 {
        rad.to_degrees()
    }
}
