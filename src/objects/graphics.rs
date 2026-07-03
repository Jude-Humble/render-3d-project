pub const TO_RADIANS: f32 = 180.0 / 3.14159;
pub const RESOLUTION: [i32; 2] = [ 1920, 1080 ];
pub const FOV: f32 = 60.0 * TO_RADIANS;
pub const CAM_OFFSET: [i32; 2] = [RESOLUTION[0] / 4, RESOLUTION[1] / 4];

use crate::math::vec3::Vec3f;

pub fn perspectivate(reference: &Vec3f, n: f32) -> [f32; 2] {
    [
        ( reference.x * n ) / ( reference.z + n ) + CAM_OFFSET[0] as f32,
        ( reference.y * n ) / ( reference.z + n ) + CAM_OFFSET[1] as f32,
    ]
}
