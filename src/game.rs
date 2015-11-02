use render::Renderer;

use std::io::Cursor;
use std::rc::Rc;

use glium::{Frame, DrawError};
use glium::backend::Facade;
use glium::texture::Texture2d;
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

        let texture = Texture2d::new(display, image).unwrap();
        let texture = Rc::new(texture);
        g.renderer.add_fullscreen_image_group(display, texture.clone())
            .unwrap();

        g
    }

    pub fn render(&self, target: &mut Frame) -> Result<(), DrawError> {
        self.renderer.render(target)
    }
}
