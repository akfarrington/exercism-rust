pub fn factors(mut n: u64) -> Vec<u64> {
    let original_n = n;
    let mut factors = Vec::new();

    for i in 2..=n {
        while n % i == 0 {
            factors.push(i);
            n /= i;

            if factors.iter().product::<u64<>>() == original_n {
                return factors;
            }
        }
    }

    factors
}