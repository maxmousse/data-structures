use std::collections::HashMap;

/// Sub array with 0 sum
///
/// https://www.geeksforgeeks.org/problems/subarray-with-0-sum-1587115621/1
///
/// Given an array of integers. Find if there is a subarray (of size at-least one) with 0 sum.
/// You just need to return true/false depending upon whether there is a subarray
/// present with 0-sum or not. Printing will be taken care by the driver code.
///
/// Example 1:
///
/// Input:
/// n = 5
/// arr = {4,2,-3,1,6}
/// Output:
/// Yes
/// Explanation:
/// 2, -3, 1 is the subarray with sum 0.
///
///
/// Solutions: https://www.geeksforgeeks.org/find-if-there-is-a-subarray-with-0-sum/
///
/// The idea is to iterate through the array and for every element arr[i], calculate
/// the sum of elements from 0 to i (this can simply be done as sum += arr[i]). If the
/// current sum has been seen before, then there must be a zero-sum subarray. Hashing
/// is used to store the sum values so that sum can be stored quickly and find out whether
/// the current sum is seen before or not.

pub fn subarray_with_0_sum(vec: &[i32]) -> bool {
    let mut sum_hashmap = HashMap::new();
    let mut sum = 0;

    for val in vec {
        sum += val;

        if sum_hashmap.contains_key(&sum) || sum == 0 {
            return true;
        } else {
            sum_hashmap.insert(sum, true);
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subarray_with_0_sum() {
        // 0 is a window of size one, with sum 0
        assert_eq!(subarray_with_0_sum(&[4, 3, 0, 8]), true);
        // [3, 5, -8] is a window of size 3 with sum 0
        assert_eq!(subarray_with_0_sum(&[4, 3, 5, -8]), true);
        // No window of any size with sum 0
        assert_eq!(subarray_with_0_sum(&[4, 3, -6, -8]), false);
    }
}
