// my initial attempt
fn reverse_initial(input: &str) -> String {
    let mut result = String::new();
    
    for c in input.chars().rev() {
        result.push(c);
    }

    result
}

// refactored with collect
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
