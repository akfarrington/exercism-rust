pub fn is_armstrong_number(num: u32) -> bool {
    let digits = sep_digits(&num);

    digits
        .iter()
        .map(|x| x.pow(digits.len() as u32))
        .sum::<u32>()
        == num
}

fn sep_digits(num: &u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}
