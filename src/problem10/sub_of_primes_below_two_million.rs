use std::println;

pub fn get_list_of_primes() -> Vec<usize> {
    const LIMIT: usize = 5000000;
    let mut prime = 2;
    // the index are the numbers
    let mut sieve = [false; LIMIT];
    let mut prime_list: Vec<usize> = vec![prime];

    loop {
        let mut counter = 2;
        let mut product = prime * counter;
        while product < sieve.len() {
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
        if prime >= 2000000 {
            break;
        }
        prime_list.push(prime);
    }
    return prime_list;
}

pub fn sub_of_primes_below_two_million() -> usize {
    let prime_list = get_list_of_primes();
    println!("{:?}", prime_list[0]);
    let result: usize = prime_list.iter().sum();
    println!("Result {:?}", result);
    return result;
}
