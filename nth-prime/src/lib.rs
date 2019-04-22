pub fn nth(n: u32) -> u32 {
    // help me find the nth
    let mut index: u32 = 0;
    // first prime is 2, so start from here
    let mut number: u32 = 2;
    // use this because of the while thing and the += stuff going on below
    let mut toreturn: u32 = 0;

    while index <= n {
        if is_prime(&number){
            toreturn = number;
            index += 1;
            number += 1;
        } else {
            number += 1;
        }
    }

    toreturn
}

pub fn is_prime(n: &u32) -> bool {
    // thanks https://exercism.io/tracks/rust/exercises/nth-prime/solutions/65a44ef3d18a4e7e9fa85605eecb3fc8
    ! (2..n - 1).any(|i| n % i == 0)
}