mod objects;
mod math;

use macroquad::prelude::*;
use crate::objects::Cube;
use crate::math::rotmat::Rotma3f;
use crate::objects::graphics::*;

pub const cam_step: f32 = 0.25;

fn pan_cam (cam_pos: &mut [f32; 2]) {
    if is_key_down(KeyCode::Up) {
        cam_pos[1] -= cam_step;
    }
    if is_key_down(KeyCode::Down) {
        cam_pos[1] += cam_step;
    }
    if is_key_down(KeyCode::Right) {
        cam_pos[0] += cam_step;
    }
    if is_key_down(KeyCode::Left) {
        cam_pos[0] -= cam_step;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "cube 3D".to_owned(),
        window_width: RESOLUTION[0],
        window_height: RESOLUTION[1],
        ..Default::default()
    }
}

#[macroquad::main("Test Window")]
async fn main() {
    

    let n = ( RESOLUTION[1] as f32) * ( 1.0 / ( FOV / 2.0 ).tan() ); 
    let mut test: Cube = Cube::new(40.0, 5.0, 0.0, 0.0, 100.0);
    let mut cam: [f32; 2] = INIT_CAM_OFFSET;
    
    let step = 0.25; // pixels per second
    let mut transform_mode: bool = false;
    let mut rotate_mode: bool = !transform_mode;

    let tester: Rotma3f = Rotma3f::x_rot(1.0);
    println!("{:?}", tester);

    loop {

        let dt = get_frame_time();
        clear_background(BLACK);
        test.render(n, cam);

        pan_cam(&mut cam);

        if is_key_down(KeyCode::E) {
            transform_mode ^= true; //xor gate bool flip
            rotate_mode ^= true; //xor gate bool flip
        }

        if transform_mode {
            if is_key_down(KeyCode::W) {
                test.transform(0.0, -step, 0.0);
            }
            if is_key_down(KeyCode::S) {
                test.transform(0.0, step, 0.0);
            }
            if is_key_down(KeyCode::A) {
                test.transform(-step, 0.0, 0.0);
            }
            if is_key_down(KeyCode::D) {
                test.transform(step, 0.0, 0.0);
            }
        } else {
            test.rotate(-0.001, 1);
        }

        next_frame().await
    }
}
