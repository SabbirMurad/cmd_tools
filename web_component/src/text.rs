use crate::helper;

pub enum ComponentType {
    Default,
    Event,
    Property,
    Attribute,
    Slot,
}

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
r#":host {{
    all: initial;
    display: block;
}}"#)
}

pub fn component_js_content(
    component_name: &str,
    component_type: ComponentType
) -> String {
    let class_name = helper::dash_to_camel_case(component_name, true);
    let export_name = helper::dash_to_camel_case(component_name, false);

    let content = match component_type {
        ComponentType::Default => "".to_string(),
        ComponentType::Event | ComponentType::Property => {
            r#"<h1 class="title">${this.title}</h1>
                <h1 class="subtitle">${this.subtitle}</h1>"#.to_string()
        },
        ComponentType::Attribute => {
            r#"<button>Emit Event</button>"#.to_string()
        },
        ComponentType::Slot => {
            r#"<h1 class="title">${this.title}</h1>"#.to_string()
        },
    };

    let property_top = match component_type {
        ComponentType::Property => {
            r#"#data = {
        title: 'Happy Coding...',
        subTitle: 'Let\'s have some fun while at it...'
    }

    get data() { return this.#data }
    set data(value) {
        /*
        IMPORTANT:
        Check and Sanitize your incoming data before use
        */
        this.#data = value

        this.#update() /* Updates the DOM */
    }
    
    "#.to_string()
        },
        _ => "".to_string()
    };

    let property_bottom = match component_type {
        ComponentType::Property => {
            r#"

    #update() {
        this.shadow.querySelector(".title").textContent = this.#data.title
        this.shadow.querySelector(".subtitle").textContent = this.#data.subTitle
    }"#.to_string()
        },
        _ => "".to_string()
    };

    let event_constructor = match component_type {
        ComponentType::Event => {
            r#"

        this.shadow.querySelector('button').addEventListener('click', () => {
            this.#emitEvent('my-event-name', {
                "hello": "World!"
            })
        })

        document.addEventListener(eventName, (event) => {
            console.info(event.detail)
        })"#.to_string()
        },
        _ => "".to_string()
    };

    let event_bottom = match component_type {
        ComponentType::Event => {
            r#"

    #emitEvent(eventName, eventData) {
        this.dispatchEvent(
            new CustomEvent(eventName, {
                bubbles: true,
                cancelable: false,
                composed: true,
                detail: eventData
            })
        )
    }"#.to_string()
        },
        _ => "".to_string()
    };

    let attribute_bottom = match component_type {
        ComponentType::Attribute => {
            r#"
    
    #updateTitle(value) {
        this.shadow.querySelector(".title").textContent = value
    }

    #updateSubTitle(value) {
        this.shadow.querySelector(".subtitle").textContent = value
    }

    /*
      Please add whatever attributes you wish to track
      from outside in the array below!
    */
    static get observedAttributes() { return ['title', 'subtitle'] }

    attributeChangedCallback(name, oldValue, newValue) {
        console.info(name, oldValue, newValue)
        switch (name) {
            case 'title':
                this.title = newValue
                this.#updateTitle(newValue)
                break;
            case 'subtitle':
                this.subtitle = newValue
                this.#updateSubTitle(newValue)
                break;
            default:
                break;
        }
    }"#.to_string()
        },
        _ => "".to_string()
    };

format!(
r#"class {class_name} extends HTMLElement {{
    {property_top}constructor() {{
        super()
        this.shadow = this.attachShadow({{ mode: "closed" }})
        this.shadow.appendChild(this.#render()){event_constructor}
    }}
    
    #render() {{
        let template = document.createElement("template")
        template.innerHTML = /*html*/`
            <link rel="stylesheet" href="/components/{component_name}/style.css">
            <div id="content_wrapper">
                {content}
            </div>
        `
        return template.content
    }}

    /*
        Get's called when the element is added to the DOM
        Remove this method if not used in production!
    */
    connectedCallback() {{
        console.info(`${{this.constructor.name}} is connected`)
    }}

    /*
        Get's called when the element is removed from the DOM
        Remove this method if not used in production!
    */
    disconnectedCallback() {{
        console.info(`${{this.constructor.name}} is disconnected`)
    }}{property_bottom}{event_bottom}{attribute_bottom}
}}

export const {export_name} = {{
    mount: function () {{
        customElements.define("{component_name}", {class_name})
    }},
    unmount: function (index) {{
        index
        ? document.querySelectorAll("{component_name}")[index].remove()
        : document.querySelector("{component_name}").remove()
    }},
}}"#)
}