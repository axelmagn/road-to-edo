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

use glium::{DisplayBuild, Surface};
use glium::glutin;


const DEFAULT_CONF_PATH: &'static str = "conf";
const DEFAULT_SETTINGS_CONF: &'static str = "settings.toml";


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

    let game = game::Game::new(&settings);


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
        target.clear_color(0.0, 0.3, 0.8, 1.0);
        game.render(&mut target);
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
