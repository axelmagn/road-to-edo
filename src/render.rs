use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use nalgebra::{Vec2, Mat4, Eye};
use glium::{self, Surface, DrawError, Program, ProgramCreationError, Frame};
use glium::backend::Facade;
use glium::draw_parameters::DrawParameters;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::Texture2d;
use glium::vertex::{VertexBuffer, BufferCreationError};
use toml;


#[derive(Copy, Clone)]
pub struct Vertex {
    /// The position of the vertex in worldspace
    pub position: Vec2<f32>,
    /// The texture coordinate of the vertex
    pub tex_coords: Vec2<f32>,
}

implement_vertex!(Vertex, position, tex_coords);

pub struct RenderGroup {
    /// verts to render
    pub vertices: VertexBuffer<Vertex>,
    /// Texture atlas
    pub atlas: Rc<Texture2d>,
    pub indices: NoIndices,
}

pub struct Renderer {
    render_groups: Vec<RenderGroup>,
    program: Program,
    // params: DrawParameters,
}

// keys for settings
pub const RENDER_SETTINGS_KEY: &'static str = "render";
pub const VERTEX_SHADER_KEY: &'static str = "shaders.vertex";
pub const FRAGMENT_SHADER_KEY: &'static str = "shaders.fragment";

/// The renderer contains graphics-related information
impl Renderer {
    /// create a new renderer
    pub fn new<F>(display: &F, settings: &toml::Table) -> Renderer 
        where F: Facade {
            let program = Renderer::load_program(
                display, &settings[RENDER_SETTINGS_KEY])
                .unwrap();
            Renderer {
                render_groups: Vec::new(),
                program: program,
            }
        }

    /// utility function to load a shader string from file
    fn load_shader_string(key: &str, settings: &toml::Value) 
        -> Result<String, String> {
            match settings.lookup(key) {
                Some(&toml::Value::String(ref v)) => {
                    let mut f = match File::open(v) {
                        Ok(f) => f,
                        Err(_) => return Err(
                            format!("Could not open shader file {}", v)),
                    };
                    let mut buf = String::new();
                    match f.read_to_string(&mut buf) {
                        Ok(_) => (),
                        Err(_) => return Err(
                            format!("Could not read shader file {}", v)),
                    };
                    Ok(buf)
                },
                Some(_) => Err(format!("{} value is not a string", key)),
                None => Err(format!("{} value is not set", key)),
            }
        }

    /// Load a glium program from settings
    fn load_program<F>(facade: &F, settings: &toml::Value) 
        -> Result<Program, ProgramCreationError> where F: Facade {

            // TODO: come back at some point and fix return value so that
            // settings error are a different error.
            let vertex_shader = Renderer::load_shader_string(
                VERTEX_SHADER_KEY, &settings);
            let vertex_shader = match vertex_shader {
                Ok(v) => v,
                Err(e) => return Err(
                    ProgramCreationError::CompilationError(e)),
            };

            let fragment_shader = Renderer::load_shader_string(
                FRAGMENT_SHADER_KEY, &settings);
            let fragment_shader = match fragment_shader {
                Ok(v) => v,
                Err(e) => return Err(
                    ProgramCreationError::CompilationError(e)),
            };

            Program::from_source(facade, &vertex_shader, &fragment_shader, 
                                 None)
        }

    /// Render all render groups to a frame
    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        target.clear_color(0.5, 0.5, 0.8, 1.0);
        let params = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };
        for group in self.render_groups.iter() {
            let uniform = uniform! {
                atlas: &*group.atlas,
                view_matrix: Mat4::new_identity(4),
            };
            target.draw(&group.vertices, &group.indices, &self.program,
                        &uniform, &params).unwrap();
        }
        Ok(())
    }

    /// Add a render group that is a static image
    pub fn add_fullscreen_image_group<F>(
        &mut self, display: &F, texture: Rc<Texture2d>) 
        -> Result<usize, BufferCreationError> 
        where F: Facade {
            let vertex1 = Vertex { 
                position: Vec2::new(-1.0,  1.0), 
                tex_coords: Vec2::new(0.0, 1.0) 
            };
            let vertex2 = Vertex { 
                position: Vec2::new(-1.0, -1.0), 
                tex_coords: Vec2::new(0.0, 0.0) 
            };
            let vertex3 = Vertex { 
                position: Vec2::new(1.0,  1.0), 
                tex_coords: Vec2::new(1.0, 1.0) 
            };
            let vertex4 = Vertex { 
                position: Vec2::new(1.0, -1.0), 
                tex_coords: Vec2::new(1.0, 0.0) 
            };
            let shape = vec![vertex1, vertex2, vertex3, vertex4];

            let vertex_buffer = VertexBuffer::new(display, &shape);
            let vertex_buffer = match vertex_buffer {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            let indices = NoIndices(PrimitiveType::TriangleStrip);

            let group = RenderGroup {
                vertices: vertex_buffer,
                atlas: texture,
                indices: indices,
            };
            self.render_groups.push(group);
            Ok(self.render_groups.len()-1)
        }
}
