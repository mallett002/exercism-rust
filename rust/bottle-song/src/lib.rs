use std::collections::HashMap;

fn uppercase_first_char(input: &str) -> String {
    let mut chars = input.chars();

    match chars.next() {
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new()
    }
}

fn get_verse(start: u32, stop: u32) -> String {
    let nums_to_nums = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
    ]);

    let start_as_str = uppercase_first_char(nums_to_nums.get(&start).unwrap());
    let stop_as_str = nums_to_nums.get(&stop).unwrap();
    let bottle = if stop == 1 { "bottle" } else { "bottles" };

    String::from(format!(
        "{start_as_str} green bottles hanging on the wall,\n\
        {start_as_str} green bottles hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {stop_as_str} green {bottle} hanging on the wall."
    ))
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let stop_bottles = start_bottles - take_down;

    for next_bottles in (stop_bottles..start_bottles).rev() {
        let next_verse = get_verse(start_bottles, next_bottles);

        result.push_str(next_verse.as_str());
    }

    result
}
