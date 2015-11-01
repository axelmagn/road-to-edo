use render::{RenderGroup, Renderer};

use glium::{Surface, Frame, DrawError};
use glium::backend::Facade;
use toml;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new<F>(display: &F, settings: &toml::Table) -> Game where F: Facade {
        Game { renderer: Renderer::new(display, settings) }
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(target)
    }
}
