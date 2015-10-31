use render::RenderGroup;

use glium::{Surface, Frame, DrawError};

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new() -> Game {
        Game { render_groups: Renderer::new() }
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(&mut target)
    }
}
