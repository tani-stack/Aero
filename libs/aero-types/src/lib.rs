#![no_std]
pub type Hertz = u32;
pub type Milliamps = u32;
pub type Millivolts = u32;
pub type Celsius = i16;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Quaternion {
    pub const IDENTITY: Self = Self {
        w: 1.0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Pose {
    pub pos: Vector3,
    pub att: Quaternion,
}
