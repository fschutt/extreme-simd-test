#![feature(asm)]

#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate stdsimd;
use stdsimd::simd;

mod sse_test_x86;

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: [f32; 4],
    pub y: [f32; 4],
}

pub trait SimdRotate {
    fn rotate_center(&mut self, angle: f32);
}

impl SimdRotate for Vec<Rect> {
    fn rotate_center(&mut self, angle: f32) {
        if sse_test_x86::have_sse2() {
            self.iter_mut().for_each(|rect| rect.rotate_center_simd(angle));
        } else {
            self.iter_mut().for_each(|rect| rect.rotate_center(angle));
        }
    }
}

impl Rect {

    // Rotates a rectangle around its center, no SIMD
    pub fn rotate_center(&mut self, in_angle: f32)
    {
        let center_y = ((self.y[1] - self.y[2]) * 0.5) + self.y[2];
        let center_x = ((self.x[1] - self.x[2]) * 0.5) + self.x[2];

        self.x[0] -= center_x; self.x[1] -= center_x;
        self.x[2] -= center_x; self.x[3] -= center_x;

        self.y[0] -= center_y; self.y[1] -= center_y;
        self.y[2] -= center_y; self.y[3] -= center_y;

        // calculate rotation
        let k_angle = in_angle.to_radians();
        let s = k_angle.sin();
        let c = k_angle.cos();

        let tl_x = (self.x[0] * c) - (self.y[0] * s);
        let tr_x = (self.x[1] * c) - (self.y[1] * s);
        let bl_x = (self.x[2] * c) - (self.y[2] * s);
        let br_x = (self.x[3] * c) - (self.y[3] * s);

        let tl_y = (self.x[0] * s) + (self.y[0] * c);
        let tr_y = (self.x[1] * s) + (self.y[1] * c);
        let bl_y = (self.x[2] * s) + (self.y[2] * c);
        let br_y = (self.x[3] * s) + (self.y[3] * c);

        self.x[0] = tl_x; self.x[1] = tr_x; self.x[2] = bl_x; self.x[3] = br_x;
        self.y[0] = tl_y; self.y[1] = tr_y; self.y[2] = bl_y; self.y[3] = br_y;

        self.x[0] += center_x; self.x[1] += center_x; self.x[2] += center_x; self.x[3] += center_x;
        self.y[0] += center_y; self.y[1] += center_y; self.y[2] += center_y; self.y[3] += center_y;
    }

    // Rotates the rectangle around its center, using SIMD
    pub fn rotate_center_simd(&mut self, in_angle: f32)
    {
        use simd;

        let center_y = ((self.y[0] - self.y[2]) * 0.5) + self.y[2];
        let center_x = ((self.x[1] - self.x[0]) * 0.5) + self.x[0];

        let mut simd_x_dir = simd::f32x4::load(&self.x, 0);
        let mut simd_y_dir = simd::f32x4::load(&self.y, 0);

        // move all points to origin
        simd_x_dir = simd_x_dir - simd::f32x4::splat(center_x);
        simd_y_dir = simd_y_dir - simd::f32x4::splat(center_y);

        // calculate rotation
        let k_angle = in_angle.to_radians();
        let s = k_angle.sin();
        let c = k_angle.cos();

        let mut simd_x_new = (simd_x_dir * simd::f32x4::splat(c)) - (simd_y_dir * simd::f32x4::splat(s));
        simd_y_dir = (simd_x_dir * simd::f32x4::splat(s)) + (simd_y_dir * simd::f32x4::splat(c));

        simd_x_new = simd_x_new + simd::f32x4::splat(center_x);
        simd_y_dir = simd_y_dir + simd::f32x4::splat(center_y);

        simd_x_new.store(&mut self.x, 0);
        simd_y_dir.store(&mut self.y, 0);
    }
}

fn main() {

}

