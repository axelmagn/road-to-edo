extern crate piston;
extern crate piston_window;
extern crate toml;

use std::io::{Read};
use std::fs::File;
use std::path::Path;

use piston_window::{WindowSettings, PistonWindow, clear};

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
    let window: PistonWindow = WindowSettings::new("Hello Piston!", 
                                                   (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    for e in window {
        e.draw_2d(|_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
