use unicode_segmentation::UnicodeSegmentation;

// my initial attempt
pub fn reverse_initial(input: &str) -> String {
    let mut result = String::new();
    
    for c in input.chars().rev() {
        result.push(c);
    }

    result
}

// refactored with collect
pub fn reverse_collect(input: &str) -> String {
    input.chars().rev().collect()
}

// Correct answer
// refactored with collect and to use graphemes
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
