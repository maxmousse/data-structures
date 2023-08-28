use std::{
    cmp::{max, min},
    collections::HashMap,
};

/// Find a continuous subset with the maximum sum
///
/// Scan the vector with a window of `window_size` and return
/// the maximum sum found by adding all the window values
pub fn max_sub_array_sum(vec: &Vec<i32>, window_size: usize) -> Result<i32, &str> {
    if window_size == 0 || window_size > vec.len() {
        return Err("Invalid window size");
    }

    let mut curr_sub_sum = 0;
    let mut max_sub_sum;

    // Calculate the sum for the initial window
    for i in 0..(window_size) {
        // !!! ranges exclude the max limit
        curr_sub_sum += vec[i];
    }

    // Init maxSubSum to currentSubSum
    max_sub_sum = curr_sub_sum;

    // Iterate throw the array, and rather than recalculate
    // the whole some for each window, add the new value and remove the old
    // one from the current sum
    for i in 1..(vec.len() - window_size + 1) {
        // !!! ranges exclude the max limit
        curr_sub_sum = curr_sub_sum - vec[i - 1] + vec[i + window_size - 1];
        max_sub_sum = max(curr_sub_sum, max_sub_sum);
    }

    Ok(max_sub_sum)
}

/// Return the minimal length of a contiguous subarray
/// for which the sum is greater than or equal to the
/// integer passed to the function
///
/// It moves a window through the vector
/// If window reach the end and the sum is inferior to the min value, we're done
// Else, window sum is superior or equal to min_value, save window size and shrink it
pub fn min_sub_array_len(vec: &Vec<u32>, min_value: u32) -> Option<usize> {
    // Init window variable
    let mut window_min_size = usize::MAX;
    let mut window_sum = vec[0];
    let mut window_start = 0;
    let mut window_end = 0;

    // Move window through the vector
    loop {
        println!("state:");
        println!("window min size: {}", window_min_size);
        println!("window sum: {}", window_sum);
        println!("window start: {}", window_start);
        println!("window end: {}", window_end);

        // If window reach the end and the sum is inferior to the min value, we're done
        if window_sum < min_value && window_end == (vec.len() - 1) {
            println!("break:");
            break;
        }
        // If window sum is inferior to min_value and window did not reach the end, expand the window
        if window_sum < min_value && window_end < (vec.len() - 1) {
            println!("expand");
            window_end += 1;
            window_sum += vec[window_end];
            println!("add {}", vec[window_end]);
        // Else, window sum is superior or equal to min_value, save window size and shrink it
        } else {
            println!("shrink:");
            window_min_size = min(window_min_size, window_end - window_start + 1);
            window_sum -= vec[window_start];
            window_start += 1;
            println!("remove {}", vec[window_start]);
        }
    }

    if window_min_size == usize::MAX {
        None
    } else {
        Some(window_min_size)
    }
}

/// Find the longest substring of a string that contains
/// unique characters
///
/// A sliding window is used to count the length of the substring
/// When an already seen char is reached, the start of the window is
/// moved to the next substring
pub fn find_longest_substring(str: &str) -> usize {
    // Start of the currently evaluated substring
    let mut start = 0;
    // Size of the longest evaluated substring
    let mut longest = 0;
    // Hash of seen chars.
    // Key: seen char
    // Value: index of the last occurrence of the char
    let mut seen = HashMap::new();

    // Loop through the chars of the string
    for (i, c) in str.chars().enumerate() {
        // If a seen character is reached, update start to the start of the new evaluated substring
        // (only if the char is in the evaluated substring and not before, due to `max`)
        if seen.contains_key(&c) {
            start = max(start, *seen.get(&c).unwrap() + 1);
        }

        // Update longest
        longest = max(longest, i - start + 1);

        // Update the seen hashmap
        seen.insert(c, i);
    }

    longest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_sub_array_sum_should_work() {
        assert!(max_sub_array_sum(&vec![1, 2, 3], 0) == Err("Invalid window size"));
        assert!(max_sub_array_sum(&vec![1, 2, 3], 4) == Err("Invalid window size"));
        assert!(max_sub_array_sum(&vec![1, 2, 3], 1) == Ok(3));
        assert!(max_sub_array_sum(&vec![1, 2, 3], 2) == Ok(5));
        assert!(max_sub_array_sum(&vec![1, 2, 3], 3) == Ok(6));
    }

    #[test]
    fn min_sub_array_len_should_work() {
        assert_eq!(min_sub_array_len(&vec![2, 3, 1, 2, 4, 3], 7), Some(2));
        assert_eq!(min_sub_array_len(&vec![2, 1, 6, 5, 4], 9), Some(2));
        assert_eq!(
            min_sub_array_len(&vec![3, 1, 7, 11, 2, 9, 8, 21, 62, 33, 19], 52),
            Some(1)
        );
        assert_eq!(
            min_sub_array_len(&vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 39),
            Some(3)
        );
        assert_eq!(
            min_sub_array_len(&vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 55),
            Some(5)
        );
        assert_eq!(min_sub_array_len(&vec![4, 3, 3, 8, 1, 2, 3], 11), Some(2));
        assert_eq!(
            min_sub_array_len(&vec![1, 4, 16, 22, 5, 7, 8, 9, 10], 95),
            None
        );
    }

    #[test]
    fn find_longest_substring_should_work() {
        assert_eq!(find_longest_substring(""), 0);
        assert_eq!(find_longest_substring("rithmschool"), 7);
        assert_eq!(find_longest_substring("thisisawesome"), 6);
        assert_eq!(find_longest_substring("thecatinthehat"), 7);
        assert_eq!(find_longest_substring("bbbbbb"), 1);
        assert_eq!(find_longest_substring("longestsubstring"), 8);
        assert_eq!(find_longest_substring("thisishowwedoit"), 6);
    }
}
