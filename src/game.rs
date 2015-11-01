use render::{RenderGroup, Renderer};

use std::io::Cursor;

use glium::{Surface, Frame, DrawError};
use glium::backend::Facade;
use image;
use toml;

pub struct Game {
    renderer: Renderer,
}

impl Game {
    pub fn new<F>(display: &F, settings: &toml::Table) -> Game where F: Facade {
        let mut g = Game { renderer: Renderer::new(display, settings) };
        let image = image::load(
            Cursor::new(
                &include_bytes!(
                    "../assets/tiles/lpc_atlas01/terrain_atlas.png"
                    )[..]
                ), image::PNG).unwrap();
        g.renderer.add_image_group(display, image);

        g
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(target)
    }
}
