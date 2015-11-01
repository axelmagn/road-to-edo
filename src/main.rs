extern crate nalgebra;
#[macro_use]
extern crate glium;
extern crate image;
extern crate toml;

#[macro_use]
mod render;
mod game;

use std::io::{Read, Cursor};
use std::fs::File;
use std::path::Path;

use glium::{DisplayBuild, Surface, GliumCreationError};
use glium::glutin::{self, WindowBuilder, CreationError};
use glium::backend::glutin_backend::GlutinFacade;


const DEFAULT_CONF_PATH: &'static str = "conf";
const DEFAULT_SETTINGS_CONF: &'static str = "settings.toml";

pub const WINDOW_SETTINGS_KEY: &'static str = "window";
pub const WINDOW_WIDTH_KEY: &'static str = "width";
pub const WINDOW_HEIGHT_KEY: &'static str = "height";

/// Load a glium display from settings
pub fn load_display(settings: &toml::Value) 
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

fn main() {
    // load environment config
    let settings = {
        let settings = Path::new(DEFAULT_CONF_PATH).join(DEFAULT_SETTINGS_CONF);
        let mut settings = File::open(settings).unwrap();
        let mut buf = String::new();
        settings.read_to_string(&mut buf).unwrap();
        toml::Parser::new(&buf).parse().unwrap()
    };

    // Create a new window 
    let width: u32 = settings["window"].lookup("width").unwrap()
        .as_integer().unwrap() as u32;
    let height: u32 = settings["window"].lookup("height").unwrap()
        .as_integer().unwrap() as u32;
    let display = glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium().unwrap();
    // initialize window to a blank color
    let mut target = display.draw();
    target.clear_color(0.0, 0.3, 0.8, 1.0);
    target.finish().unwrap();

    // load tile sheet
    let tiles_img = image::load(
        Cursor::new(
            &include_bytes!(
                "../assets/tiles/lpc_atlas01/terrain_atlas.png"
            )[..]
        ),
        image::PNG).unwrap();
    let tiles_tex = glium::Texture2d::new(&display, tiles_img).unwrap();

    let dest_tex = glium::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        1024, 1024).unwrap();

    let game = game::Game::new(&display, &settings);


    loop {
        // blit the whole tilemap over to dest
        dest_tex.as_surface().clear_color(0.0, 0.3, 0.8, 1.0);

        let dest_rect = glium::BlitTarget {
            left: 0,
            bottom: 0,
            width: dest_tex.get_width() as i32,
            height: dest_tex.get_height().unwrap() as i32,
        };

        tiles_tex.as_surface().blit_whole_color_to(
            &dest_tex.as_surface(), &dest_rect, 
            glium::uniforms::MagnifySamplerFilter::Linear);


        let mut target = display.draw();
        // target.clear_color(0.0, 0.3, 0.8, 1.0);
        game.render(&mut target).unwrap();
        /*
        dest_tex.as_surface().fill(
            &target, glium::uniforms::MagnifySamplerFilter::Linear);
        */

        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                _ => (),
            }
        }
    }

}
