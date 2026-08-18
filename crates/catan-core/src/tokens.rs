pub const STANDARD_TOKENS: [u8; 18] = [
    2, 3, 3, 4, 4, 5, 5, 6, 6, // no 7
    8, 8, 9, 9, 10, 10, 11, 11, 12,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eighteen_tokens() {
        let tokens = STANDARD_TOKENS;
        assert_eq!(tokens.len(), 18)
    }

    #[test]
    fn no_seven_and_in_range() {
        let valid = STANDARD_TOKENS
            .iter()
            .all(|&x| (2..=12).contains(&x) && x != 7);
        assert!(valid)
    }

    #[test]
    fn correct_multiset() {
        let counts = STANDARD_TOKENS.iter().fold([0u8; 13], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });

        assert_eq!(counts[2], 1);
        assert_eq!(counts[12], 1);

        for &n in [3, 4, 5, 6, 8, 9, 10, 11].iter() {
            assert_eq!(counts[n as usize], 2)
        }
    }
}
