use glium::{VertexBuffer, IndexBuffer, Vertex};
use glium::index::{Index, PrimitiveType, IndicesSource};
use glium::vertex::MultiVerticesSource;

#[derive(Copy, Clone, Debug)]
pub struct SpriteVertex {
    position: [f32; 2],
    texcoord: [f32; 2]
}

impl SpriteVertex {
    #[inline]
    pub fn new(pos: [f32; 2], coord: [f32; 2]) -> SpriteVertex {
        SpriteVertex {
            position: pos,
            texcoord: coord
        }
    }
}

implement_vertex!(SpriteVertex, position, texcoord);

/*
pub struct Geometry<V: Vertex, I: Index> {
    pub vertices: Vec<V>,
    pub indices: Vec<I>,
    pub primitive: PrimitiveType
}
*/

#[derive(Clone, Debug)]
pub struct BufferedGeometry<V, I> where for<'a> V: MultiVerticesSource<'a>, for<'b> I: Into<IndicesSource<'b>> {
    pub vertices: V,
    pub indices: I
}

impl<V, I> BufferedGeometry<V, I> where for<'a> V: MultiVerticesSource<'a>, for<'b> I: Into<IndicesSource<'b>> {

    pub fn new(vertices: V, indices: I) -> BufferedGeometry<V, I> {
        BufferedGeometry {
            vertices: vertices,
            indices: indices,
        }
    }

}
