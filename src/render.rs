use std::rc::Rc;

use nalgebra::Vec2;
use glium::{DisplayBuild, Surface, DrawError, Program};
use glium::draw_parameters::DrawParameters;
use glium::texture::Texture2d;
use glium::uniforms::Uniforms;
use glium::vertex::VertexBuffer;
use glium::index::{self, IndexBuffer};


#[derive(Copy, Clone)]
struct Vertex {
    /// The position of the vertex in worldspace
    position: Vec2<f32>,
    /// The texture coordinate of the vertex
    tex_coords: Vec2<f32>,
}

implement_vertex!(Vertex, position, tex_coords);


#[derive(Copy, Clone)]
struct GlobalUniforms {
    view_coords: Vec2<f32>,
    view_size: Vec2<f32>,
}

struct RenderGroupUniforms {
    /// texture atlas to use
    atlas: Texture2d,
}


struct RenderGroup {
    vertices: VertexBuffer<Vertex>,
    uniforms: RenderGroupUniforms,
}
