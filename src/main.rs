extern crate tera;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use tera::Tera;

fn main() {
    let json_file_path = Path::new("opcodes.json");
    let display = json_file_path.display();

    let mut json_file = match File::open(json_file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut json_string = String::new();
    match json_file.read_to_string(&mut json_string) {
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

    let template_name = "desassembler.rs";
    
    let render_file_path = Path::new("render");
    let path = render_file_path.join(template_name);
    let render_file = match File::create(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    match tera.render_to(template_name, &context, &render_file) {
        Ok(_) => println!("render successful"),
        Err(e) => panic!("error: {}", e),
    }
}
