pub fn help(exc_name: &str) -> String {
  format!(r#"
Available commands:

🟢 -h / help      --  Get a list of the commands you can run
🟢 -g / generate  --  Generate web components,
                      pass in the type and component name,
                      example: {} -g static my-component
  "#, exc_name)
}

pub fn command_not_found(exc_name: &str, command: &str) -> String {
format!(r#"
Command {} not found

🚫 Run: {} -help  or clay -h for help
  "#, command, exc_name)
}

pub fn component_type_and_name_required(exe_name: &str) -> String {
format!(r#"
🚫 Component type and name required!

💡 example: {} -g static my-component

Available component types:
🟢 -d || default
🟢 -e || event
🟢 -p || property
🟢 -a || attribute
🟢 -s || slot
  "#, exe_name)
}

pub fn invalid_type() -> String {
format!(r#"
🚫 Invalid generate type!

Available component types:
🟢 -d || default
🟢 -e || event
🟢 -p || property
🟢 -a || attribute
🟢 -s || slot
  "#)
}

pub fn component_already_exists(component_name: &str) -> String {
format!(r#"
🚫 Component {} already exists!
  "#, component_name)
}

// pub fn project_dir_required(command: &str) -> String {
// format!(r#"
// 🚫 Project directory is required

// Example: clay {} <project directory>
//   "#, command)
// }

// pub fn project_dir_not_found(dir: &str) -> String {
// format!(r#"
// 🚫 Project directory not found

// The directory 📁 {} does not exist
//   "#, dir)
// }

// pub fn project_dir_already_exists(dir: &str) -> String {
// format!(r#"
// 🚫 Project directory already exist

// The directory 📁 {} already exist
//   "#, dir)
// }

// pub fn clayignore() -> String {
//   format!(r#".clayignore
// .claywatch
// map.db"#)
// }

// pub fn claywatch() -> String {
//   format!(r#"assets/js
// assets/css
// map.db"#)
// }