use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rayon::prelude::*;
use std::time::Duration;

mod utils;
use utils::{Vec2, Vec3, Vec4};

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;

const MAX_STEPS: i32 = 100;
const SURFACE_DIST: f64 = 0.01;
const MAX_DIST: f64 = 100.0;

// fn s_box(p: Vec3, s: Vec3) -> f64 {
//     vec![p.abs() - s, Vec3::new(0.0, 0.0, 0.0)]
//         .into_iter()
//         .max_by(|a, b| a.magnitude().partial_cmp(&b.magnitude()).unwrap())
//         .unwrap()
//         .magnitude()
// }

fn s_sphere(p: Vec3, s: Vec4) -> f64 {
    (p - s.to_vec3()).magnitude() - s.w
}

fn get_dist(p: Vec3) -> f64 {
    let ds = s_sphere(p.clone(), Vec4::new(0.0, 1.0, 6.0, 0.75));
    // let db = s_box(
    //     p.clone() - Vec3::new(2.0, 1.0, 6.0),
    //     Vec3::new(1.0, 1.0, 1.0),
    // );

    let dp = p.clone().y;

    vec![dp, ds].iter().copied().fold(f64::INFINITY, f64::min)
}

fn get_normal(p: Vec3) -> Vec3 {
    let d = get_dist(p.clone());
    let e = Vec2::new(0.01, 0.0);
    let n = Vec3::new(d, d, d)
        - Vec3::new(
            get_dist(p.clone() - e.xyy()),
            get_dist(p.clone() - e.yxy()),
            get_dist(p.clone() - e.yyx()),
        );
    n.normalize()
}

fn get_light(p: Vec3) -> f64 {
    let light_pos = Vec3::new(-1.0, 5.0, 6.0);
    let l = (light_pos - p.clone()).normalize();
    let n = get_normal(p.clone());
    n.dot(l)
}

fn raymarch(ro: Vec3, rd: Vec3) -> f64 {
    let mut d0 = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro.clone() + rd.clone() * d0;
        let ds = get_dist(p);
        d0 += ds;
        if ds < SURFACE_DIST || d0 > MAX_DIST {
            break;
        }
    }
    return d0;
}

fn main() {
    let mut first_launch = true;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("ray2m", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::R, KeyRepeat::No) || first_launch {
            buffer
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| {
                    let i = index / WIDTH;
                    let j = index % WIDTH;

                    let x = j as f64;
                    let y = i as f64;
                    let width = WIDTH as f64;
                    let height = HEIGHT as f64;

                    let uv = (Vec2::new(x, y) - Vec2::new(width, height) * 0.5) / height;
                    let ro = Vec3::new(0.0, 1.0, 0.0);
                    let rd = Vec3::new(uv.x, uv.y, 1.0).normalize();

                    let d = raymarch(ro.clone(), rd.clone());
                    let p = ro + rd * d;
                    let dif = get_light(p);

                    let rounded_value = (255.0 / 4.0 - (dif * 100.0).round()).round() as u32;
                    *pixel = (rounded_value << 16) | (rounded_value << 8) | rounded_value;
                });

            first_launch = false;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
