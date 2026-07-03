pub mod graphics;

use crate::math::quaternion::Quaternion;
use crate::math::vec3::Vec3f;
use crate::math::rotmat::Rotma3f;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Cube {
    top: [Vec3f; 4],
    bottom: [Vec3f; 4],
    thickness: f32,
}

impl Cube {
    pub fn new(s: f32, t: f32, x_start: f32, y_start: f32, z_start: f32) -> Self {
        Self {
            top: [
                Vec3f { x:  s + x_start, y:  s + y_start, z:  s + z_start },
                Vec3f { x: -s + x_start, y:  s + y_start, z:  s + z_start },
                Vec3f { x:  s + x_start, y:  s + y_start, z: -s + z_start },
                Vec3f { x: -s + x_start, y:  s + y_start, z: -s + z_start },
            ],
            
            bottom: [
                Vec3f { x:  s + x_start, y: -s + y_start, z:  s + z_start },
                Vec3f { x: -s + x_start, y: -s + y_start, z:  s + z_start },
                Vec3f { x:  s + x_start, y: -s + y_start, z: -s + z_start },
                Vec3f { x: -s + x_start, y: -s + y_start, z: -s + z_start },
            ],

            thickness: t,
        }
    }

    pub fn transform(&mut self, x_mov: f32, y_mov: f32, z_mov: f32) {
        for i in 0..4 {
            self.top[i].x += x_mov;
            self.top[i].y += y_mov;
            self.top[i].z += z_mov;
        }

        for i in 0..4 {
            self.bottom[i].x += x_mov;
            self.bottom[i].y += y_mov;
            self.bottom[i].z += z_mov;
        }
    }

    pub fn rotate(&mut self, theta: f32, dir: i8) {
        let mut rotation_matrix: Rotma3f;

        match dir {
            0 => rotation_matrix = Rotma3f::x_rot(theta),
            1 => rotation_matrix = Rotma3f::y_rot(theta),
            2 => rotation_matrix = Rotma3f::z_rot(theta),
            _ => panic!("Invalid Direction Index For Rotation!"),
        }

        rotation_matrix.multiply(&mut self.top);
        rotation_matrix.multiply(&mut self.bottom);
    }

    pub fn render(&self, n: f32) {

        let mut buffer: Vec<[f32; 2]> = Vec::new();

        for i in 0..4 {
            buffer.push(graphics::perspectivate(&self.top[i], n));
        }
        for i in 0..4 {
            buffer.push(graphics::perspectivate(&self.bottom[i], n));
        }

        draw_line(buffer[0][0], buffer[0][1], buffer[1][0], buffer[1][1], self.thickness, WHITE); // Top-back
        draw_line(buffer[4][0], buffer[4][1], buffer[5][0], buffer[5][1], self.thickness, WHITE); // Bottom-back
        draw_line(buffer[0][0], buffer[0][1], buffer[4][0], buffer[4][1], self.thickness, WHITE); // Back-right-side
        draw_line(buffer[1][0], buffer[1][1], buffer[5][0], buffer[5][1], self.thickness, WHITE); // Back-left-side

        draw_line(buffer[2][0], buffer[2][1], buffer[3][0], buffer[3][1], self.thickness, WHITE); // Top-front
        draw_line(buffer[6][0], buffer[6][1], buffer[7][0], buffer[7][1], self.thickness, WHITE); // Bottom-front
        draw_line(buffer[2][0], buffer[2][1], buffer[6][0], buffer[6][1], self.thickness, WHITE); // Front-right-side
        draw_line(buffer[3][0], buffer[3][1], buffer[7][0], buffer[7][1], self.thickness, WHITE); // Front-left-side

        draw_line(buffer[0][0], buffer[0][1], buffer[2][0], buffer[2][1], self.thickness, WHITE); // Top-right-corner
        draw_line(buffer[1][0], buffer[1][1], buffer[3][0], buffer[3][1], self.thickness, WHITE); // Top-left-corner
        draw_line(buffer[4][0], buffer[4][1], buffer[6][0], buffer[6][1], self.thickness, WHITE); // Bottom-right-corner
        draw_line(buffer[5][0], buffer[5][1], buffer[7][0], buffer[7][1], self.thickness, WHITE); // Bottom-left-corner
    }
}
