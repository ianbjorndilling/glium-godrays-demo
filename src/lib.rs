#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

pub mod entity;
pub mod sprite;
pub mod camera;
pub mod geometry;

use std::io::{Seek, Read};
use glium::backend::Facade;
use glium::texture::{Texture2d, RawImage2d};
use image::ImageFormat;

pub fn create_texture<F: Facade, T: Seek + Read>(facade: &F, source: T, format: ImageFormat) -> Texture2d {
    let img = image::load(source, format).unwrap().to_rgba();
    let size = img.dimensions();
    Texture2d::new(facade, RawImage2d::from_raw_rgba_reversed(img.into_raw(), size)).unwrap()
}

/*
use cgmath::Matrix4;
pub fn projection2d(width: f32, height: f32, depth: f32) -> Matrix4<f32> {
    Matrix4::new(2. / width, 0., 0., 0.,
                 0., -2. / height, 0., 0.,
                 0., 0., 2. / depth, 0.,
                 -1., 1., 0., 1.)
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
