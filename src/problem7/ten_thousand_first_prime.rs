use core::panic;
use std::println;

pub fn sieve_of_eratosthenes(position: usize) -> usize {
    const LIMIT: usize = 500000;
    let mut prime_number_position = 1;
    let mut prime = 2;
    // the index are the numbers
    let mut sieve = [false; LIMIT];

    while prime_number_position < position {
        let mut counter = 2;
        let mut product = prime * counter;
        // println!(
        //     "On position {:?} with prime {:?}",
        //     prime_number_position, prime
        // );
        while product < sieve.len() {
            // println!("Product {:?}", product);
            sieve[product] = true;
            counter += 1;
            product = prime * counter;
        }
        prime = sieve
            .iter()
            .skip(prime + 1)
            .position(|x| !*x)
            .unwrap_or_else(|| panic!("LIMIT TOO LOW!"))
            + prime
            + 1;
        prime_number_position += 1;
    }
    println!("Prime at position {:?} is {:?}", position, prime);
    return prime;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_prime_at_nth() {
        assert_eq!(sieve_of_eratosthenes(1), 2);
        assert_eq!(sieve_of_eratosthenes(2), 3);
        assert_eq!(sieve_of_eratosthenes(3), 5);
        assert_eq!(sieve_of_eratosthenes(4), 7);
        assert_eq!(sieve_of_eratosthenes(5), 11);
        assert_eq!(sieve_of_eratosthenes(6), 13);
        assert_eq!(sieve_of_eratosthenes(7), 17);
        assert_eq!(sieve_of_eratosthenes(8), 19);
    }
}
