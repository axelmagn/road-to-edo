use render::{RenderGroup, Renderer};

use glium::{Surface, Frame, DrawError};
use toml;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new(settings: &toml::Table) -> Game {
        Game { renderer: Renderer::new(settings) }
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(target)
    }
}
