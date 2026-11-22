use std::{cmp, println};

fn is_palindrome(number: usize) -> bool {
    let number_as_string = number.to_string();
    let number_length = number_as_string.len();
    let mut half_string: Vec<char> = Vec::new();
    // if even go half way
    // if odd go one before midpoint
    for (i, char) in number_as_string.chars().take(number_length / 2).enumerate() {
        half_string.insert(i, char);
    }
    // now reverse iterate and make sure values exist in the vector
    for (j, char) in number_as_string
        .chars()
        .rev()
        .take(number_length / 2)
        .enumerate()
    {
        if half_string[j] != char {
            return false;
        }
    }
    return true;
}

pub fn largest_palindrome_product() {
    let mut max_palindrome = 0;
    for i in 100..999 {
        for j in 100..999 {
            let product = i * j;
            if is_palindrome(product) {
                max_palindrome = cmp::max(max_palindrome, product);
            }
        }
    }
    println!("{:?}", max_palindrome);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(1234), false);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(90109), true);
        assert_eq!(is_palindrome(0000), true);
    }
}
