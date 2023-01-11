extern crate tera;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use tera::Tera;

fn main() {
    let json_file_path = Path::new("opcodes.json");
    let display = json_file_path.display();

    let mut file = match File::open(json_file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut json_string = String::new();
    match file.read_to_string(&mut json_string) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let json = match serde_json::from_str(&json_string) {
        Err(why) => panic!("couldn't parse {}: {}", display, why),
        Ok(json) => json,
    };

    let context = match tera::Context::from_value(json) {
        Err(why) => panic!("couldn't transform {} to tera context: {}", display, why),
        Ok(context) => context,
    };

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => panic!("Parsing error(s): {}", e),
    };

    match tera.render("desassembler.rs", &context) {
        Ok(s) => print!("{}", s),
        Err(e) => panic!("error: {}", e),
    }
}
