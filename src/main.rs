extern crate piston;
extern crate toml;

use std::fs::File;
use std::path::Path;

use piston::WindowSettings;

const DEFAULT_CONF_PATH: Path = Path::new("conf");
const DEFAULT_SETTINGS_CONF: Path = Path::new("engine.toml");

fn main() {
    // load environment config
    let settings = {
        let settings = DEFAULT_CONF_PATH.join(DEFAULT_SETTINGS_CONF);
        let settings = File::open(settings);
        let mut buf = String::new();
        settings.read_to_string(&mut buf).unwrap();
        toml::Parser::new(&buf).parse().unwrap()
    };
    println!("{:?}", settings);
    
    // Create a new window 
}
