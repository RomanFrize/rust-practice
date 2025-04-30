fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}


fn main() {
    let examples = ["Hello", "Привет", "123aB!", "RUST"];
    for input in examples {
        let result = invert_the_case(input.to_string());
        println!("Original: {:<10} | Inverted: {}", input, result);
    }
}