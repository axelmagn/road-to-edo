extern crate glium;
extern crate toml;

use std::io::{Read};
use std::fs::File;
use std::path::Path;

use glium::{DisplayBuild, Surface};

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
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .build_glium().unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.3, 0.5, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }

}
