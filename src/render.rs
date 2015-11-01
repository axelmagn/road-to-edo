use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use nalgebra::{Vec2, Mat4, Identity, Eye};
use glium::{self, DisplayBuild, Surface, DrawError, Program, 
    ProgramCreationError, Frame, GliumCreationError};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::glutin::{WindowBuilder, CreationError};
use glium::index::{self, IndexBuffer, NoIndices, IndicesSource};
use glium::texture::Texture2d;
use glium::uniforms::Uniforms;
use glium::vertex::VertexBuffer;
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
    pub atlas: Texture2d,
    pub indices: NoIndices,
}

pub struct Renderer {
    render_groups: Vec<RenderGroup>,
    program: Program,
    // params: DrawParameters,
}

pub const RENDER_SETTINGS_KEY: &'static str = "render";
pub const VERTEX_SHADER_KEY: &'static str = "shaders.vertex";
pub const FRAGMENT_SHADER_KEY: &'static str = "shaders.fragment";

/// The renderer contains graphics-related information
impl Renderer {
    pub fn new<F>(display: &F, settings: &toml::Table) -> Renderer where F: Facade {
        let program = Renderer::load_program(display, 
                                             &settings[RENDER_SETTINGS_KEY])
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





    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        target.clear_color(0.5, 0.5, 0.8, 1.0);
        for group in self.render_groups.iter() {
            let uniform = uniform! {
                atlas: &group.atlas,
                view_matrix: Mat4::new_identity(4),
            };
            // target.draw(&group.vertices, 
        }
        Ok(())
    }
}
