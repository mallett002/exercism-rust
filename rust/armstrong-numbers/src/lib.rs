// armstrong:
// N is the number: ex 51
// D: number of digits is the power (ex 51 has 2 digits. so power is 2)
// X: take each digit to the power of D and add them up (51 -> 5^2 + 1^2 = 10 + 1 = 11)
// If X = N, it's an armstrong number

// Ex: 153 is an Armstrong number, because: 153 = ( 1^3 + 5^3 + 3^3 ) = ( 1 + 125 + 27 ) = 153
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    let digit_len = digits.len() as u32;
    let mut built_num: u32 = 0;

    for digit in digits {
        built_num += digit.pow(digit_len)        
    }

    built_num == num
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    while num > 0 {
        let digit = num % 10;
        digits.push(digit);
        num /= 10;
    }

    digits.reverse();

    digits
}
