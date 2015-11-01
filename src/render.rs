use std::rc::Rc;
use std::fs::File;
use std::io::Read;

use nalgebra::Vec2;
use glium::{DisplayBuild, Surface, DrawError, Program, ProgramCreationError, 
    Frame, GliumCreationError};
use glium::backend::Facade;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::glutin::{WindowBuilder, CreationError};
use glium::index::{self, IndexBuffer};
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
    display: GlutinFacade,
    program: Program,
    // params: DrawParameters,
}

pub const RENDER_SETTINGS_KEY: &'static str = "render";
pub const WINDOW_SETTINGS_KEY: &'static str = "window";
pub const VERTEX_SHADER_KEY: &'static str = "shaders.vertex";
pub const FRAGMENT_SHADER_KEY: &'static str = "shaders.fragment";
pub const WINDOW_WIDTH_KEY: &'static str = "width";
pub const WINDOW_HEIGHT_KEY: &'static str = "height";

/// The renderer contains graphics-related information
impl Renderer {
    pub fn new(settings: &toml::Table) -> Renderer {
        let display = Renderer::load_display(&settings[WINDOW_SETTINGS_KEY])
            .unwrap();
        let program = Renderer::load_program(&display, 
                                             &settings[RENDER_SETTINGS_KEY])
            .unwrap();
        Renderer {
            render_groups: Vec::new(),
            display: display,
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


    /// Load a glium display from settings
    fn load_display(settings: &toml::Value) 
        -> Result<GlutinFacade, GliumCreationError<CreationError>> {
            // TODO: come back at some point and fix return value so that we
            // don't have to panic if we can't read settings
            let width: u32 = settings.lookup(WINDOW_WIDTH_KEY)
                .and_then(|v| v.as_integer())
                .unwrap() as u32;
            let height: u32 = settings.lookup(WINDOW_HEIGHT_KEY)
                .and_then(|v| v.as_integer())
                .unwrap() as u32;
            WindowBuilder::new()
                .with_dimensions(width, height)
                .build_glium()
        }



    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        target.clear_color(1.0, 0.3, 0.8, 1.0);
        for group in self.render_groups.iter() {
        }
        Ok(())
    }
}
