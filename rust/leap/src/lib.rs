pub fn is_leap_year_first_attempt(year: u64) -> bool {
    if year % 100 == 0 {
        if year % 400 == 0 {
            return true;
        }

        return false;
    }

    if year % 4 == 0 {
        return true;
    }

    false
}

// Using match
pub fn is_leap_year(year: u64) -> bool {
    match year {
        y if y % 400 == 0 => true,
        y if y % 100 == 0 => false,
        y if y % 4 == 0 => true,
        _ => false,
    }
}
