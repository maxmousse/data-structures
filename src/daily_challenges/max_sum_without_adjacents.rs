use std::cmp::max;

/// Max sum without adjacents
///
/// Given an array Arr of size N containing positive integers.
/// Find the maximum sum of a any possible subsequence such that
/// no two numbers in the subsequence should be adjacent in Arr.
///
/// Example 1:
///
/// Input:
/// N = 6
/// Arr[] = {5, 5, 10, 100, 10, 5}
/// Output: 110
/// Explanation: If you take indices 0, 3
/// and 5, then Arr[0]+Arr[3]+Arr[5] =
/// 5+100+5 = 110.
///
/// Example 2:
///
/// Input:
/// N = 4
/// Arr[] = {3, 2, 7, 10}
/// Output: 13
/// Explanation: 3 and 10 forms a non
/// continuous subsequence with maximum
/// sum.
///
/// Expected Time Complexity: O(N)
/// Expected Auxiliary Space: O(1)
///
/// Constraints:
/// 1 ≤ N ≤ 105
/// 1 ≤ Arri ≤ 105
pub fn collect_max(vec: &[u32], buffer: &mut Vec<Option<u32>>, i: usize, n: usize) -> u32 {
    if i >= n {
        return 0;
    }

    if i == (n - 1) {
        return vec[i];
    }

    if buffer[i] != None {
        return buffer[i].unwrap();
    }

    let result = max(
        vec[i] + collect_max(&vec, buffer, i + 2, n),
        collect_max(&vec, buffer, i + 1, n),
    );

    buffer[i] = Some(result);

    result
}

pub fn find_max(vec: &[u32]) -> u32 {
    let n = vec.len();
    let mut buffer = vec![None::<u32>; n];
    max(
        collect_max(&vec, &mut buffer, 0, n),
        collect_max(&vec, &mut buffer, 1, n),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[5, 5, 10, 100, 10, 5]), 110);
        assert_eq!(find_max(&[3, 2, 7, 10]), 13);
        assert_eq!(find_max(&[3, 2, 5, 10, 7]), 15);
    }
}
