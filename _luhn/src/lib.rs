/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    const RADIX: u32 = 10;

    // check if the code is too short
    if code.trim().len() < 2 {
        return false;
    }

    // check if any chars are not a space or number, and if so return false
    if code
        .chars()
        .any(|character| !character.is_numeric() && !character.eq(&' '))
    {
        return false;
    }

    // filter out spaces, collect into Vec<u8>, reverse to find
    // what numbers to double, double the correct ones, find sum
    let sum: u8 = code
        .chars()
        .filter(|character| character.is_numeric())
        .map(|character| character.to_digit(RADIX).unwrap() as u8)
        .collect::<Vec<u8>>()
        .iter()
        .rev()
        .enumerate()
        .map(|(x, &number)| match x {
            0 => number,
            1 => double(number),
            _ => match x % 2 == 0 {
                true => number,
                false => double(number),
            },
        })
        .sum();

    sum % 10 == 0
}

// doubles the number then subtracts 9 if it's greater than 9
fn double(number: u8) -> u8 {
    let doubled = number * 2;
    match doubled > 9 {
        true => doubled - 9,
        false => doubled,
    }
}
