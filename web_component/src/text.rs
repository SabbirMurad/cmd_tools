use create::helper;

pub fn help(exc_name: &str) -> String {
  format!(r#"
Available commands:

🟢 -h / help      --  Get a list of the commands you can run
🟢 -g / generate  --  Generate web components,
                      pass in the type and component name,
                      example: {} -g static my-component

Visit https://sabbirhassan.com/documentation for more information
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

pub fn component_css_content() -> String {
format!(
r#":host {
    all: initial;
    display: block;
}"#)
}

pub fn component_js_content(component_name: &str) -> String {
    let class_name = helper::dash_to_camel_case(component_name, true);
    let export_name = helper::dash_to_camel_case(component_name, false);

format!(
r#"class {class_name} extends HTMLElement {
    constructor() {
        super()
        this.shadow = this.attachShadow({ mode: "closed" })
        this.shadow.appendChild(this.#render())
    }
    
    #render() {
        let template = document.createElement("template")
        template.innerHTML = /*html*/`
            <link rel="stylesheet" href="/components/{component_name}/style.css">
            <div class="spinner_box">
                <svg class="spinner" viewBox="0 0 50 50">
                    <circle class="path" cx="25" cy="25" r="20" fill="none" stroke-width="4"></circle>
                </svg>
            </div>
        `
        return template.content
    }
}

export const {export_name} = {
    mount: function () {
        customElements.define("{component_name}", {class_name})
    },
    unmount: function (index) {
        index
        ? document.querySelectorAll("{component_name}")[index].remove()
        : document.querySelector("{component_name}").remove()
    },
}"#)
}