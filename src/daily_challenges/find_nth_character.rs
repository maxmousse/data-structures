/// Find the N-th character
///
/// https://www.geeksforgeeks.org/problems/find-the-n-th-character5925/1
///
/// Given a binary string s. Perform r iterations on string s, where in each iteration 0 becomes 01 and 1 becomes 10. Find the nth character (considering 0 based indexing) of the string after performing these r iterations (see examples for better understanding).
///
/// Example 1:
///
/// Input:
/// s = "1100"
/// r = 2
/// n = 3
/// Output:
/// 1
/// Explanation:
/// After 1st iteration s becomes "10100101".
/// After 2nd iteration s becomes "1001100101100110".
/// Now, we can clearly see that the character at 3rd index is 1, and so the output.
///
/// Example 2:
///
/// Input:
/// s = "1010"
/// r = 1
/// n = 2
/// Output:
/// 0
/// Explanation :
/// After 1st iteration s becomes "10011001".
/// Now, we can clearly see that the character at 2nd index is 0, and so the output.
///
/// Your task:
/// You don't need to read input or print anything. Your task is to complete the function nthCharacter() which takes the string s and integers r and n as input parameters and returns the n-th character of the string after performing r operations on s.
///  
/// Expected Time Complexity: O(r*|s|)
/// Expected Auxilary Space: O(|s|)
///  
/// Constraints:
/// 1 ≤ |s| ≤ 103
/// 1 ≤ r ≤ 20
/// 0 ≤ n < |s|
pub fn find_nth_character(value: &str, iteration_number: u32, char_position: usize) -> char {
    let mut value: String = value.to_string();
    let mut temp_value = String::new();

    // Note: a each iteration, `value` length double. However, it is useless
    // to calculate the chars after `char_position` as they won't be used and
    // do not influence the result
    // That' why we go out from inner loop as soon as `value` is long enough
    for _ in 0..iteration_number {
        for c in value.chars() {
            match c {
                '0' => temp_value.push_str("01"),
                '1' => temp_value.push_str("10"),
                _ => panic!("Invalid character in input value"),
            }

            // Leave early if temp value is long enough
            if temp_value.len() > char_position {
                break;
            }
        }

        value = temp_value;
        temp_value = String::new();
    }

    value.chars().nth(char_position).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_nth_character() {
        assert_eq!(find_nth_character("1100", 2, 3), '1');
        assert_eq!(find_nth_character("1010", 1, 2), '0');
    }
}
