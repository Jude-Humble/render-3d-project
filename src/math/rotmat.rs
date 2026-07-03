use crate::math::vec3::Vec3f;

#[derive(Debug)]
pub struct Rotma3f {
    mat: [Vec3f; 3],
}

impl Rotma3f {
    pub fn x_rot(theta: f32) -> Self {
        Self {
            mat: [
                Vec3f { x: 1.0, y: 0.0, z: 0.0 },
                Vec3f { x: 0.0, y: theta.cos(), z: -theta.sin() },
                Vec3f { x: 0.0, y: theta.sin(), z: theta.cos() },
            ]
        }
    }

    pub fn y_rot(theta: f32) -> Self {
        Self {
            mat: [
                Vec3f { x: theta.cos(), y: 0.0, z: theta.sin() },
                Vec3f { x: 0.0, y: 1.0, z: 0.0},
                Vec3f { x: -theta.sin(), y: 0.0, z: theta.cos() },
            ]
        }
    }

    pub fn z_rot(theta: f32) -> Self {
        Self {
            mat: [
                Vec3f { x: theta.cos(), y: -theta.sin(), z: 0.0 },
                Vec3f { x: theta.sin(), y: theta.cos(), z: 0.0 },
                Vec3f { x: 0.0, y: 0.0, z: 1.0 },
            ]
        }
    }

    pub fn multiply(&mut self, to_mult: &mut [Vec3f; 4]) {
        for i in 0..4 {
            to_mult[i] = Vec3f {
                x: 
                    self.mat[0].x * to_mult[i].x + 
                    self.mat[0].y * to_mult[i].y + 
                    self.mat[0].z * to_mult[i].z,
                y:
                    self.mat[1].x * to_mult[i].x +
                    self.mat[1].y * to_mult[i].y +
                    self.mat[1].z * to_mult[i].z,
                z:
                    self.mat[2].x * to_mult[i].x +
                    self.mat[2].y * to_mult[i].y +
                    self.mat[2].z * to_mult[i].z,
            };
        }
    }
}
