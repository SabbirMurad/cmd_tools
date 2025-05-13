use std::io::Write;
use std::{env, fs, process};
use std::path::Path;

mod text;
mod helper;

const ARGS: [&str; 4] = [
    "-h",
    "help",
    "-g",
    "generate",
];

const GENERATE_TYPES: [&str; 10] = [
    "-d",
    "default",
    "-e",
    "event",
    "-p",
    "property",
    "-a",
    "attribute",
    "-s",
    "slot",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let exe_name = &args[0];

    if args.len() < 2 {
        println!("{}", text::help(exe_name));
        process::exit(0);
    }

    let command = &args[1];

    if !ARGS.contains(&command.as_str()) {
        println!("{}", text::command_not_found(exe_name, command));
        process::exit(0);
    }

    if command == "help" || command == "-h" {
        println!("{}", text::help(exe_name));
        process::exit(0);
    }

    if command == "generate" || command == "-g" {
        if args.len() < 4 {
            println!("{}", text::component_type_and_name_required(exe_name));
            process::exit(0);
        }

        let component_type = &args[2];
    
        if !GENERATE_TYPES.contains(&component_type.as_str()) {
            println!("{}", text::invalid_type());
            process::exit(0);
        }
    
        let component_name = &args[3];

        /*
            Creating the root component folder if not already exists
        */
        let component_root_folder = Path::new("component");
        if !component_root_folder.exists() {
            let result = fs::create_dir("component");
            if let Err(e) = result {
                println!("Error creating directory: {}", e);
                process::exit(0);
            }
        }

        /*
            Creating the component folder if not already exists, error if already exists
        */
        let folder_name = format!("components/{}", component_name.clone());
        let component_folder = Path::new(&folder_name);
    
        if component_folder.exists() {
            println!("{}", text::component_already_exists(component_name));
            process::exit(0);
        }

        let result = fs::create_dir(&folder_name);
        if let Err(e) = result {
            println!("Error creating directory: {}", e);
            process::exit(0);
        }

        /*
            Creating the javascript file 
        */
        let js_file_path = format!("{}/module.js", folder_name);
        let mut js_file = match fs::File::create(js_file_path) {
            Ok(file) => file,
            Err(why) => {
                println!("! {}", why);
                process::exit(0);
            },
        };

        let component_type = match component_type.as_str() {
            "-d" | "default" => text::ComponentType::Default,
            "-e" | "event" => text::ComponentType::Event,
            "-p" | "property" => text::ComponentType::Property,
            "-a" | "attribute" => text::ComponentType::Attribute,
            "-s" | "slot" => text::ComponentType::Slot,
            _ => text::ComponentType::Default,
        };
        /*
            Writing to the javascript file
        */
        if let Err(why) = js_file.write_all(
            text::component_js_content(
                component_name,
                component_type
            ).as_bytes()
        ) {
            println!("! {}", why);
            process::exit(0);
        };

        /*
            Creating the css file 
        */
        let css_file_path = format!("{}/style.css", folder_name);
        let mut css_file = match fs::File::create(css_file_path) {
            Ok(file) => file,
            Err(why) => {
                println!("! {}", why);
                process::exit(0);
            },
        };

        /*
            Writing to the css file
        */
        if let Err(why) = css_file.write_all(
            text::component_css_content().as_bytes()
        ) {
            println!("! {}", why);
            process::exit(0);
        };

        println!("Successfully generated!!!");
    }
}