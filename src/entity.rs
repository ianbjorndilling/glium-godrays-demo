use cgmath::{Vector3, Vector2, Rad, Matrix4, Angle};

pub trait Transform {
    fn translate(&mut self, offset: Vector2<f32>);
    fn rotate(&mut self, angle: Rad<f32>);
    fn scale(&mut self, scalar: f32);
    fn nonuniform_scale(&mut self, x: f32, y: f32);
}

#[inline]
pub fn model_matrix(transl: Vector3<f32>, rot: Rad<f32>, scale: Vector2<f32>) -> Matrix4<f32> {
    Matrix4::from_translation(transl) *
            rotation_2d(rot) *
            Matrix4::from_nonuniform_scale(scale.x, scale.y, 1.)
}

#[inline]
pub fn rotation_2d(angle: Rad<f32>) -> Matrix4<f32> {
    let (s, c) = angle.sin_cos();
    Matrix4::new(c, -s, 0., 0.,
                 s, c, 0., 0.,
                 0., 0., 1., 0.,
                 0., 0., 0., 1.)
}
