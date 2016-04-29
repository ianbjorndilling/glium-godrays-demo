use cgmath::{
    Rad,
    Point3,
    Vector2,
    Vector3,
    Matrix4,
    Array,
    EuclideanSpace,
    SquareMatrix
};

use super::entity::{model_matrix, Transform};

pub struct Camera {
    pub position: Point3<f32>,
    pub rotation: Rad<f32>,
    pub scale: Vector2<f32>
}

impl Camera {

    #[inline]
    pub fn new() -> Camera {
        Camera {
            position: Point3::from_value(0.),
            rotation: Rad::new(0.),
            scale: Vector2::from_value(1.)
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        model_matrix(-self.position.to_vec(), -self.rotation, self.scale)
    }

}

impl Transform for Camera {

    #[inline]
    fn translate(&mut self, offset: Vector2<f32>) {
        self.position = self.position + Vector3::new(offset.x, offset.y, 0.);
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
