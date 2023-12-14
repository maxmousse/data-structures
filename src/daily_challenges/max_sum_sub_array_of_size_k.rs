use std::cmp::max;

/// Given an array of integers Arr of size N and a number K. Return the maximum sum of a subarray of size K.
///
/// (https://www.geeksforgeeks.org/problems/max-sum-subarray-of-size-k5313/1)
///
/// NOTE*: A subarray is a contiguous part of any given array.
///
/// Example 1:
///
/// Input:
/// N = 4, K = 2
/// Arr = [100, 200, 300, 400]
/// Output:
/// 700
/// Explanation:
/// Arr3  + Arr4 =700,
/// which is maximum.
///
/// Example 2:
///
/// Input:
/// N = 4, K = 4
/// Arr = [100, 200, 300, 400]
/// Output:
/// 1000
/// Explanation:
/// Arr1 + Arr2 + Arr3 + Arr4 =1000,
/// which is maximum.
///
/// Your Task:
///
/// You don't need to read input or print anything. Your task is to complete the function maximumSumSubarray() which takes the integer K, vector Arr with size N, containing the elements of the array and returns the maximum sum of a subarray of size K.
///
/// Expected Time Complexity: O(N)
/// Expected Auxiliary Space: O(1)
///
/// Constraints:
/// 1 <= N <= 105
/// 1 <= Arri <= 105
/// 1 <= K <= N
pub fn max_sum_sub_array_of_size_k(vec: &[u32], window_size: usize) -> Option<u32> {
    // Quick exit if vec is smaller than window_size
    if window_size == 0 || window_size > vec.len() {
        return None;
    }

    let mut window_sum = 0;

    // Sum of initial window
    for i in 0..window_size {
        window_sum += vec[i];
    }

    let mut max_window_sum = window_sum;

    // Scan vec with window to find max_window_sum
    for i in window_size..vec.len() {
        window_sum = window_sum + vec[i] - vec[i - window_size];
        max_window_sum = max(max_window_sum, window_sum);
    }

    Some(max_window_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sum_sub_array_of_size_k() {
        // Window size == 0
        assert_eq!(max_sum_sub_array_of_size_k(&[1, 2, 3], 4), None);

        // Window size > vec.len()
        assert_eq!(max_sum_sub_array_of_size_k(&[1, 2, 3], 4), None);

        // Window size == vec.len()
        assert_eq!(max_sum_sub_array_of_size_k(&[1, 2, 12, 5, 6], 5), Some(26));

        // Window size == 1
        assert_eq!(max_sum_sub_array_of_size_k(&[1, 2, 12, 5, 6], 1), Some(12));

        // 1 < window_size < vec.len()
        assert_eq!(max_sum_sub_array_of_size_k(&[1, 2, 12, 5, 6], 3), Some(23));
    }
}
