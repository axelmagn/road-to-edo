use std::rc::Rc;
use std::fs::File;

use nalgebra::Vec2;
use glium::{DisplayBuild, Surface, DrawError, Program, Facade, 
    ProgramCreationError};
use glium::draw_parameters::DrawParameters;
use glium::texture::Texture2d;
use glium::uniforms::Uniforms;
use glium::vertex::VertexBuffer;
use glium::index::{self, IndexBuffer};
use toml;


#[derive(Copy, Clone)]
pub struct Vertex {
    /// The position of the vertex in worldspace
    pub position: Vec2<f32>,
    /// The texture coordinate of the vertex
    pub tex_coords: Vec2<f32>,
}

implement_vertex!(Vertex, position, tex_coords);


#[derive(Copy, Clone)]
pub struct GlobalUniforms {
    pub view_coords: Vec2<f32>,
    pub view_size: Vec2<f32>,
}

pub struct RenderGroupUniforms {
    /// texture atlas to use
    pub atlas: Rc<Texture2d>,
}


pub struct RenderGroup {
    pub vertices: VertexBuffer<Vertex>,
    pub uniforms: RenderGroupUniforms,
}

pub struct Renderer {
    render_groups: Vec<RenderGroup>,
    program: Program,
    params: DrawParameters,
}

pub const VERTEX_SHADER_KEY: &'static str = "render.shaders.vertex";
pub const FRAGMENT_SHADER_KEY: &'static str = "fragment.shaders.vertex";

/// The renderer contains graphics-related information
impl Renderer {
    pub fn new(settings: &toml::Table) -> Renderer {
        Renderer {
            render_groups: Vec::new(),
        }
    }

    /// utility function to load a shader string from file
    fn load_shader_string(key: &str, settings: &toml::Table) 
        -> Result<String, String> {
            match settings.lookup(key) {
                Some(String(v)) => {
                    let f = match File::open(v) {
                        Ok(f) => f,
                        Err(_) => return Err(
                            format!("Could not open shader file {}", v)),
                    };
                    let mut buf = String::new();
                    match f.read_to_string(&buf) {
                        Ok(_) => (),
                        Err(_) => return Err(
                            format!("Could not read shader file {}", v)),
                    };
                    buf
                },
                Some(_) => Err(format!("{} value is not a string", key)),
                None => Err(format!("{} value is not set", key)),
            }
        }

    pub fn make_program<F>(facade: &F, settings: &toml::Table) 
        -> Result<Program, ProgramCreationError> where F: Facade {
            let vetex_shader = load_shader_string(
                VERTEX_SHADER_KEY, &settings)
                .or_else(|e| Err(ProgramCreationError::CompilationError(e)));
            let vertex_shader = match vertex_shader {
                Ok(v) => v,
                Err(e) => return vertex_shader
            };
            let fragment_shader = load_shader_string(FRAGMENT_SHADER_KEY, 
                                                           &settings);
            let fragment_shader = match fragment_shader {
                Ok(v) => v,
                Err(e) => return Err(
                    ProgramCreationError::CompilationError(e)),
            };

        }


    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        target.clear_color(1.0, 0.3, 0.8, 1.0);
        for group in self.render_groups.iter() {
        }
        Ok(())
    }
}
