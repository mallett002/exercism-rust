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
        (0, "no"),
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

    let start_bottle = if start == 1 { "bottle" } else { "bottles" };
    let stop_bottle = if stop == 1 { "bottle" } else { "bottles" };

    String::from(format!(
        "{start_as_str} green {start_bottle} hanging on the wall,\n\
        {start_as_str} green {start_bottle} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {stop_as_str} green {stop_bottle} hanging on the wall.\n"
    ))
}

#[deprecated(since = "always", note = "Use `recite` instead")]
pub fn recite2(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let stop_bottles = start_bottles - take_down;
    let mut where_to_start = start_bottles;

    for next_bottles in (stop_bottles..start_bottles).rev() {
        let next_verse = get_verse(where_to_start, next_bottles);

        result.push_str(next_verse.as_str());

        where_to_start -= 1;

        if next_bottles != stop_bottles {
            result.push_str("\n\n");
        }
    }

    result
}



/*
start_bottles:  10
take_down:      2 (how many prints)

10, 9

let count = 0

while count > take_down:
    count += 1;
    print(start_bottles);
*/

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut current_bottles = start_bottles;
    let mut verse_count = 0;

    let mut result = String::new();

    while verse_count < take_down {
        verse_count += 1;

        result.push_str(get_verse(current_bottles, current_bottles - 1).as_str());

        if verse_count != take_down {
            result.push_str("\n");
        }

        current_bottles -= 1;
    }

    result
}





