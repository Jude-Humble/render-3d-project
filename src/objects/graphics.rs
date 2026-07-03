pub const TO_RADIANS: f32 = 180.0 / 3.14159;
pub const RESOLUTION: [i32; 2] = [ 1920, 1080 ];
pub const FOV: f32 = 60.0 * TO_RADIANS;
pub const INIT_CAM_OFFSET: [f32; 2] = [0.0,0.0];

use crate::math::vec3::Vec3f;

pub fn perspectivate(reference: &Vec3f, n: f32, cam_offset: [f32; 2]) -> [f32; 2] {
    [
        ( reference.x * n ) / ( reference.z + n ) - cam_offset[0],
        ( reference.y * n ) / ( reference.z + n ) - cam_offset[1],
    ]
}
