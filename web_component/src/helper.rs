pub fn dash_to_camel_case(input: &str, first_cap: bool) -> String {
    let mut result = String::new();
    let mut capitalize_next = first_cap;

    for (i, c) in input.chars().enumerate() {
        if c == '-' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                // Keep the first character lowercase
                if i == 0 {
                    result.push(c.to_lowercase().next().unwrap());
                } else {
                    result.push(c);
                }
            }
        }
    }
    
    result
}