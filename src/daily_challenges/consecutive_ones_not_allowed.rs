/// Consecutive 1's not allowed
///
/// Given a positive integer N, count all possible distinct binary strings
/// of length N such that there are no consecutive 1’s.
/// Output your answer modulo 109 + 7.
///
/// Example 1:
///
/// Input:
/// N = 3
/// Output: 5
/// Explanation:
/// 5 strings are (000,
/// 001, 010, 100, 101).
///
/// Example 2:
///
/// Input:
/// N = 2
/// Output: 3
/// Explanation:
/// 3 strings are (00,01,10).
///
///
/// Expected Time Complexity: O(N)
/// Expected Auxiliary Space: O(N)
///
/// Constraints:
/// 1 ≤ N ≤ 105
pub fn consecutive_ones_not_allowed(n: u32) -> u32 {
    // Explanations
    //
    // For a binary string of size n, let say we have:
    // - combinations_starting_with_0(n) distinct binary string without consecutive 1, that starts with 0
    // - combinations_starting_with_1(n) distinct binary string without consecutive 1, that starts with 1
    //
    // Example: n = 2
    // combinations_starting_with_0(2) = 2 ('00', '01)
    // combinations_starting_with_1(2) = 1 ('10' only, as '11' is not valid)
    //
    // For a string of size n + 1 ( <=> to add a digit in front of the allowed combinations for a string of size n)
    // combinations_starting_with_0(n+1) = combinations_starting_with_0(n) + combinations_starting_with_1(n) (as all combinations of size n stay valid if we add a 0 at the beginning)
    // combinations_starting_with_1(n+1) = combinations_starting_with_0(n) (as only combinations of size n, that start with a 0 stay valid if we add a 1 at the beginning)
    let mut i = 1;
    let mut combinations_starting_with_0 = 1;
    let mut combinations_starting_with_1 = 1;
    let mut total_combinations = combinations_starting_with_0 + combinations_starting_with_1;

    // Determine combinations_starting_with_0(i) / combinations_starting_with_1(i) until we reach n
    while i < n {
        combinations_starting_with_1 = combinations_starting_with_0;
        combinations_starting_with_0 = total_combinations;
        total_combinations = combinations_starting_with_0 + combinations_starting_with_1;
        i += 1;
    }

    total_combinations
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consecutive_ones_not_allowed() {
        assert_eq!(consecutive_ones_not_allowed(1), 2);
        assert_eq!(consecutive_ones_not_allowed(2), 3);
        assert_eq!(consecutive_ones_not_allowed(3), 5);
        assert_eq!(consecutive_ones_not_allowed(4), 8);
        assert_eq!(consecutive_ones_not_allowed(5), 13);
    }
}
