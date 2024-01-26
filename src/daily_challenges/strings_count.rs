/// String's count
///
/// Given a length n, count the number of strings of length n that can
/// be made using a, b and c with at-most one b and two c allowed.
///
/// Example 1:
///
/// Input: n = 1
/// Output: 3
/// Explanation: Possible strings are: "a",
/// "b" and "c"
///
/// Example 2:
///
/// Input: n = 3
/// Output: 19
/// Explanation: Number of strings with 3
/// occurrences of a: 1
/// 2-a and 1-b: 3
/// 2-a and 1-c: 3
/// 1-a, 1-b and 1-c: 6
/// 1-a and 2-c: 3
/// 1-b and 2-c: 3
/// So, total number of strings of length 3
/// is 1 + 3 + 3 + 6 + 3 + 3 = 19
///
/// Expected Time Complexity: O(1)
/// Expected Auxiliary Space: O(1)
///
/// Constraints:
/// 1 ≤ n ≤ 105
pub fn strings_count(n: u32) -> u32 {
    match n {
        0 => 0,
        _ => {
            let strings_with_a_only = 1;
            let strings_with_one_b = n;
            let strings_with_one_c = n;
            let strings_with_two_c = n * (n - 1) / 2;
            // strings with two b =  n-1 + ... 2 + 1;
            // cc___
            // c_c__
            // c__c_
            // c___c

            // cc___ x
            // _cc__
            // _c_c_
            // _c__c

            // c_c__ x
            // _cc__ x
            // __cc_
            // __c_c

            // c__c_ x
            // _c_c_ x
            // __cc_ x
            // ___cc

            let strings_with_one_b_and_one_c = n * (n - 1);
            let strings_with_one_b_and_two_c = if n > 2 {
                strings_with_two_c * (n - 2)
            } else {
                0
            };

            strings_with_a_only
                + strings_with_one_b
                + strings_with_one_c
                + strings_with_two_c
                + strings_with_one_b_and_one_c
                + strings_with_one_b_and_two_c
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strings_count() {
        assert_eq!(strings_count(0), 0);
        assert_eq!(strings_count(1), 3);
        assert_eq!(strings_count(2), 8);
        assert_eq!(strings_count(3), 19);
        assert_eq!(strings_count(4), 39);
        assert_eq!(strings_count(5), 71);
        assert_eq!(strings_count(6), 118);
    }
}
