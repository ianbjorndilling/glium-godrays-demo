#[macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;
extern crate pi_arcade;

use std::io::Cursor;

use glium::{
    Surface,
    DisplayBuild,
    Program,
    VertexBuffer,
    IndexBuffer,
    glutin
};

use glium::texture::Texture2d;

use pi_arcade::create_texture;
use pi_arcade::sprite::{Sprite};
use pi_arcade::geometry::SpriteVertex;
use pi_arcade::camera::Camera;
use pi_arcade::entity::Transform;

static SCREEN_VERT: &'static str = include_str!("./glsl/screen.vert.glsl");

static SPRITE_VERT: &'static str = include_str!("./glsl/sprite.vert.glsl");
static SPRITE_FRAG: &'static str = include_str!("./glsl/sprite_silhouette.frag.glsl");

static SHAPE_VERT: &'static str = include_str!("./glsl/shape.vert.glsl");
static SHAPE_FRAG: &'static str = include_str!("./glsl/shape.frag.glsl");

static OCCL_FRAG: &'static str = include_str!("./glsl/sprite_occlusion.frag.glsl");
static LIGHT_FRAG: &'static str = include_str!("./glsl/radial_gradient.frag.glsl");

static GODRAY_FRAG: &'static str = include_str!("./glsl/godray.frag.glsl");

struct KeyState {
    left: bool,
    right: bool
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            left: false,
            right: false
        }
    }

    pub fn update(&mut self, key: glutin::VirtualKeyCode, state: glutin::ElementState) {
        match key {
            glutin::VirtualKeyCode::Left => self.update_left(state),
            glutin::VirtualKeyCode::Right => self.update_right(state),
            _ => ()
        };
    }

    fn update_left(&mut self, state: glutin::ElementState) {
        if self.left && state == glutin::ElementState::Released {
            self.left = false;
            return;
        }
        self.left = true;
    }

    fn update_right(&mut self, state: glutin::ElementState) {
        if self.right && state == glutin::ElementState::Released {
            self.right = false;
            return;
        }
        self.right = true;
    }

}

fn main() {

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1500, 1000)
        .with_title(String::from("Arcade"))
        .with_vsync()
        .build_glium()
        .unwrap();

    let sprite_program = glium::Program::from_source(&display, SPRITE_VERT, SPRITE_FRAG, None).unwrap();
    let occl_program = glium::Program::from_source(&display, SPRITE_VERT, OCCL_FRAG, None).unwrap();
    let shape_program = glium::Program::from_source(&display, SHAPE_VERT, SHAPE_FRAG, None).unwrap();
    let light_program = glium::Program::from_source(&display, SCREEN_VERT, LIGHT_FRAG, None).unwrap();
    let godray_program = glium::Program::from_source(&display, SCREEN_VERT, GODRAY_FRAG, None).unwrap();

    let texture = create_texture(&display, Cursor::new(&include_bytes!("./resources/Untitled.png")[..]), image::PNG);

    let mut sprite1 = Sprite::new(&texture, [0.0, 0.0, 0.0]);
    let mut sprite2 = Sprite::new(&texture, [0.0, 0.0, 0.0]);
    let mut sprite3 = Sprite::new(&texture, [0.0, 0.0, 0.0]);
    sprite1.depth = 900.;
    sprite1.translate(cgmath::Vector2::new(0., -55.));
    sprite2.depth = 1400.;
    sprite2.translate(cgmath::Vector2::new(600., -200.0));
    sprite3.depth = 1700.;
    sprite3.translate(cgmath::Vector2::new(-900., -300.0));

    let sprites = vec![sprite1, sprite2, sprite3];

    let mut camera = Camera::new();
    camera.translate(cgmath::Vector2::new(0., 0.));

    let quad = [
        SpriteVertex::new([-1.0, -1.0], [0.0, 0.0]),
        SpriteVertex::new([-1.0, 1.0], [0.0, 1.0]),
        SpriteVertex::new([1.0, 1.0], [1.0, 1.0]),
        SpriteVertex::new([1.0, -1.0], [1.0, 0.0])
    ];

    let ground_quad = [
        SpriteVertex::new([-1.0, -1.0], [0.0, 0.0]),
        SpriteVertex::new([-1.0, -0.5], [0.0, 1.0]),
        SpriteVertex::new([1.0, -0.5], [1.0, 1.0]),
        SpriteVertex::new([1.0, -1.0], [1.0, 0.0])
    ];

    let indices: [u8; 6] = [
        0, 1, 2,
        0, 2, 3
    ];

    let ground_buffer = glium::VertexBuffer::new(&display, &ground_quad).unwrap();
    let sprite_buffer = glium::VertexBuffer::new(&display, &quad).unwrap();
    let ibuffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let depth_max = 10000f32;

    let mut keystate = KeyState::new();

    loop {

        if keystate.left {
            camera.translate(cgmath::Vector2::new(-1.5, 0.0));
        }

        if keystate.right {
            camera.translate(cgmath::Vector2::new(1.5, 0.0));
        }

        let mut screen = display.draw();
        let sdem = screen.get_dimensions();
        let (w, h) = (sdem.0 as f32, sdem.1 as f32);

        let proj: [[f32; 4]; 4] = cgmath::perspective(cgmath::Deg::new(60.), w / h, 0.01f32, depth_max).into();
        //let proj: [[f32; 4]; 4] = cgmath::ortho(-w / 2., w / 2., -h / 2., h / 2., 0., 100.).into();
        let view: [[f32; 4]; 4] = camera.view_matrix().into();

        let occl_map = Texture2d::empty(&display, sdem.0 / 2, sdem.1 / 2).unwrap();
        let mut occl_target = occl_map.as_surface();
        occl_target.clear_color(0.0, 0.0, 0.0, 1.0);

        screen.clear_color(0.0, 0.04, 0.08, 1.0);

        // draw to occlusion map
        {
           let uniforms = uniform! {
               color_from: [0.6f32, 1.0, 1.0],
           };
           let params = glium::DrawParameters {
               blend: glium::Blend::alpha_blending(),
               ..Default::default()
           };
           occl_target.draw(&sprite_buffer, &ibuffer, &light_program, &uniforms, &params).unwrap();
        }

        for sprite in &sprites {
            let model: [[f32; 4]; 4] = sprite.get_matrix().into();
            let uniforms = uniform! {
                projection: proj,
                view: view,
                model: model,
                depth: sprite.depth,
                sprite: sprite.texture
            };
            let params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            occl_target.draw(&sprite_buffer, &ibuffer, &occl_program, &uniforms, &params).unwrap();
        }

        {
            let uniforms = uniform!{
                light_position: [0.5f32, 0.5],
                occlusion: glium::uniforms::Sampler::new(&occl_map).magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
                exposure: 0.2f32,
                decay: 0.968f32,
                density: 0.926f32,
                weight: 0.58767f32
            };
            let params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };
            screen.draw(&sprite_buffer, &ibuffer, &godray_program, &uniforms, &params).unwrap();

        }

        for sprite in &sprites {
            let model: [[f32; 4]; 4] = sprite.get_matrix().into();

            let uniforms = uniform! {
                projection: proj,
                view: view,
                model: model,
                depth: sprite.depth,
                sprite: sprite.texture,
                color: [0f32, 0.0, 0.0],
                depth_max: depth_max
            };

            let params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            screen.draw(&sprite_buffer, &ibuffer, &sprite_program, &uniforms, &params).unwrap();
        }

        {

            let uniforms = uniform! {
                depth: 1f32,
                bottom_offset: 0.0f32,
                color: [0f32, 0.0, 0.0],
                depth_max: depth_max
            };
            screen.draw(&ground_buffer, &ibuffer, &shape_program, &uniforms, &Default::default()).unwrap();
        }



        screen.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                // resized does not trigger on OSX
                // glutin::Event::Resized(w, h) => println!("{} x {}", w, h),
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(state, _, code) => keystate.update(code.unwrap(), state),
                _ => ()
            }
        }

        std::thread::sleep(std::time::Duration::new(0, 30));
    }

}

