use std::collections::HashMap;

/// Check if frequencies can be equal
///
/// Given a string s which contains only lower alphabetic characters, check if it is possible to remove at most one character from this string in such a way that frequency of each distinct character becomes same in the string. Return true if it is possible to do else return false.
///
/// Note: The driver code print 1 if the value returned is true, otherwise 0.
///
/// Example 1:
///
/// Input:
/// s = "xyyz"
/// Output:
/// 1
/// Explanation:
/// Removing one 'y' will make frequency of each character to be 1.
///
/// Example 2:
///
/// Input:
/// s = "xxxxyyzz"
/// Output:
/// 0
/// Explanation:
/// Frequency can not be made same by removing at most one character.
///
/// Your Task:  
/// You dont need to read input or print anything. Complete the function sameFreq() which takes a string as input parameter and returns a boolean value denoting if same frequency is possible or not.
///
/// Expected Time Complexity: O(|s|)
/// Expected Auxiliary Space: O(1)
///
/// Constraints:
/// 1 <= |s| <= 10^5
pub fn can_frequencies_be_equal(input: &str) -> bool {
    let frequencies: HashMap<char, u32> = input.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let frequencies_distribution: HashMap<u32, u32> =
        frequencies.values().fold(HashMap::new(), |mut acc, val| {
            *acc.entry(*val).or_insert(0) += 1;
            acc
        });

    if frequencies_distribution.keys().len() == 2 {
        let mut keys: Vec<&u32> = frequencies_distribution.keys().collect();
        keys.sort();
        let v1 = keys[0];
        let v2 = keys[1];

        if *v2 == v1 + 1 {
            if frequencies_distribution[v2] == 1 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_frequencies_be_equal() {
        assert_eq!(can_frequencies_be_equal("xyyz"), true);
        assert_eq!(can_frequencies_be_equal("xxxxyyzz"), false);
    }
}
