use cgmath::{Rad, Array, Point2, Vector2, Vector3, Matrix4};
use glium::texture::Texture2d;
use glium::{Surface, Program, DrawParameters};
use glium::backend::Facade;
use super::entity::{model_matrix, Transform};
use super::geometry::BufferedGeometry;
use super::camera::Camera;

/*
static SPRITE_VERT: &'static str = include_str!("./glsl/sprite.vert.glsl");
static SPRITE_FRAG: &'static str = include_str!("./glsl/sprite_silhouette.frag.glsl");

pub struct SpriteProgram<'a> {
    program: Program,
    params: DrawParameters<'a>,
}

impl SpriteProgram {

    #[inline]
    pub fn new<F: Facade>(facade: &F) {
        SpriteProgram {
            program: Program::from_source(&display, SPRITE_VERT, SPRITE_FRAG, None).unwrap(),
            params: DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            }
        }

    }

}
*/

pub struct Sprite<'a> {
    pub position: Point2<f32>,
    pub rotation: Rad<f32>,
    pub scale: Vector2<f32>,
    pub texture: &'a Texture2d,
    pub color: [f32; 3],
    pub depth: f32
}

impl<'a> Sprite<'a> {

    #[inline]
    pub fn new(tex: &'a Texture2d, color: [f32; 3]) -> Sprite {
        Sprite {
            position: Point2::from_value(0.),
            rotation: Rad::new(0.),
            scale: Vector2::new(tex.get_width() as f32, tex.get_height().unwrap() as f32),
            texture: tex,
            color: color,
            depth: 1.
        }
    }

    #[inline]
    pub fn get_matrix(&self) -> Matrix4<f32> {
        model_matrix(Vector3::new(self.position.x, self.position.y, 0.0), self.rotation, self.scale)
    }



}


impl<'a> Transform for Sprite<'a> {

    #[inline]
    fn translate(&mut self, offset: Vector2<f32>) {
        self.position = self.position + offset;
    }

    #[inline]
    fn rotate(&mut self, angle: Rad<f32>) {
        self.rotation = self.rotation + angle;
    }

    #[inline]
    fn scale(&mut self, scalar: f32) {
        self.scale = self.scale * scalar;
    }

    #[inline]
    fn nonuniform_scale(&mut self, x: f32, y: f32) {
        self.scale.x *= x;
        self.scale.y *= y;
    }

}

/*
pub struct SpriteLayer<'a> {
    pub depth: f32,
    pub sprites: Vec<&'a Sprite<'a>>
}

impl<'a> SpriteLayer<'a> {

    pub fn new(depth: f32) -> SpriteLayer {
        SpriteLayer {
            depth: depth,
            sprites: Vec::new()
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite<'a>) {
        self.sprites.push(sprite);
    }

}
*/

/*
pub struct Scene<'a> {
    pub width: f32,
    pub height: f32,
    pub max_depth: f32,
    pub layers: Vec<&'a SpriteLayer<'a>>,
    pub geom: BufferedGeometry<'a>,
    pub camera: &'a Camera,
}

impl<'a> Scene<'a> {

    pub fn new(camera: Camera, max_depth: f32, sprite_geom: BufferedGeometry<'a>) -> Scene {
        Scene {
            max_depth: max_depth,
            layers: Vec::new()
        }
    }

    pub fn draw<S: Surface>(&self, surface: &S, program: &SpriteProgram) {
        let proj: [[f32; 4]; 4] = cgmath::ortho(-w / 2., w / 2., -h / 2., h / 2., 0., 100.).into();
        let view: [[f32; 4]; 4] = camera.view_matrix().into();
        let model: [[f32; 4]; 4] = sprite.get_matrix().into();

        let uniforms = uniform! {
            projection: proj,
            view: view,
            model: model,
            depth: ,
            sprite: sprite.texture,
            color: [0f32, 0.0, 0.0],
            depth_max: 10f32
        };
        screen.draw(buff.vertices, buff.indices, program.program, uni, &program.params).unwrap();
    }

}
*/
