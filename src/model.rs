use wgpu;

pub struct Model {

}

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub struct Material {

}
