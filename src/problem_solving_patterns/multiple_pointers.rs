/// Find the first pair for which the sum result is zero
///
/// Given a sorted array of integers, look for the first pair of
/// integers for which the sum is 0.
/// It uses 2 pointers (one initialized at the beginning, one at the end)
/// that moves smartly to the center of the vector
pub fn sum_zero(vec: Vec<i32>) -> Option<(i32, i32)> {
    // Handle empty vector case
    if vec.len() == 0 {
        return None;
    }

    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = vec.len() - 1;

    // Iterates through the array until the two pointers meet
    while left_pointer < right_pointer {
        // If there is a pair, return
        if vec[left_pointer] + vec[right_pointer] == 0 {
            return Some((vec[left_pointer], vec[right_pointer]));
        }
        // If sum is positive, move rightPointer to a lower value
        if vec[left_pointer] + vec[right_pointer] > 0 {
            right_pointer = right_pointer - 1;
        }
        // If sum is negative move leftPointer to a bigger value
        if vec[left_pointer] + vec[right_pointer] < 0 {
            left_pointer = left_pointer + 1;
        }
    }
    // If nothing found, return None
    None
}

/// Look for a pair which have the targeted mean
///
/// Given a sorted array of integers and a target average,
/// determine if there is a pair of values
/// in the array where the average of the pair equals
///  the target average. There may be more than one pair
///  that matches the average target.
///
/// Expected complexity:
/// Time: O(N)
/// Space: O(1)
///
/// Sample Input:
///     averagePair([1,2,3],2.5) // true
///     averagePair([1,3,3,5,6,7,10,12,19],8) // true
///     averagePair([-1,0,3,4,5,6], 4.1) // false
///     averagePair([],4) // false
pub fn average_pair(values: Vec<i32>, target: f32) -> bool {
    // Guard against vector that do not have enough values
    if values.len() < 2 {
        return false;
    }

    // Init pointers
    let mut left_pointer = 0;
    let mut right_pointer = values.len() - 1;

    // Move pointers smartly until a matching average is found or until
    // the pointers meet
    while left_pointer != right_pointer {
        let average = (values[left_pointer] + values[right_pointer]) as f32 / 2.0;

        match average {
            a if (a as f32) == target => return true,
            a if (a as f32) > target => right_pointer -= 1,
            a if (a as f32) < target => left_pointer += 1,
            _ => return false,
        }
    }

    false
}

/// Takes two strings and checks whether the characters in the first string
/// form a subsequence of the characters in the second string.
///
/// In other words, the function should check whether the characters in the first
/// string appear somewhere in the second string, without their order changing.
///
/// Examples:
///     isSubsequence('hello', 'hello world'); // true
///     isSubsequence('sing', 'sting'); // true
///     isSubsequence('abc', 'abracadabra'); // true
///     isSubsequence('abc', 'acb'); // false (order matters)
///
/// Time Complexity - O(N + M)
/// Space Complexity - O(1)
pub fn is_subsequence(str1: &str, str2: &str) -> bool {
    // Subsequence of empty string is always true
    if str1.len() == 0 {
        return false;
    }

    let mut i = 0;
    let mut j = 0;

    // Loop throw all str2 to look for substring
    while j < str2.len() {
        // When a char from str1 is found in str2, look for the next
        // char of str1
        if str1.chars().nth(i) == str2.chars().nth(j) {
            i += 1;
        }
        // When all chars of str1 has been found, return true
        if i == str1.len() {
            return true;
        }
        // Continue if an end condition was not meet
        j += 1;
    }
    // return false if str1 is not a subsequence of str2
    false
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn sum_zero_should_work() {
        let valid = vec![-1, 0, 1];
        let invalid = vec![-1, 0, 2];
        let empty: Vec<i32> = vec![];

        assert!(sum_zero(valid) == Some((-1, 1)));
        assert!(sum_zero(invalid) == None);
        assert!(sum_zero(empty) == None);
    }

    #[test]
    fn average_pair_should_work() {
        assert!(average_pair(vec![1, 2, 3], 2.5) == true);
        assert!(average_pair(vec![1, 3, 3, 5, 6, 7, 10, 12, 19], 8.0) == true);
        assert!(average_pair(vec![-1, 0, 3, 4, 5, 6], 4.1) == false);
        assert!(average_pair(vec![], 4.0) == false);
    }

    #[test]
    fn is_subsequence_should_work() {
        assert!(is_subsequence("hello", "hello world") == true);
        assert!(is_subsequence("sing", "sting") == true);
        assert!(is_subsequence("abc", "abracadabra") == true);
        assert!(is_subsequence("abc", "acb") == false); // (order matters)
    }
}
