use render::{RenderGroup, Renderer};

use glium::{Surface, Frame, DrawError};

pub struct Game {
    // renderer: Renderer,
    foo: f64,
}

impl Game {
    pub fn new() -> Game {
        Game { renderer: Renderer::new() }
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(&mut target)
    }
}
