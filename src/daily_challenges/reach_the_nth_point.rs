/// Reach the nth point
///
/// There are N points on the road, you can step ahead by 1 or 2 .
/// If you start from a point 0, and can only move from point i to point
/// i+1 after taking a step of length one, find the number of ways you
/// can reach at point N.
///
/// Example 1:
///
/// Input:
/// N = 4
/// Output:
/// 5
/// Explanation: Three ways to reach at 4th
/// point. They are {1, 1, 1, 1}, {1, 1, 2},
/// {1, 2, 1} {2, 1, 1}, {2, 2}.
///
/// Example 2:
///
/// Input: N = 5
/// Output: 8
/// Explanation: Three ways to reach at 5th
/// point. They are {1, 1, 1, 1, 1},
/// {1, 1, 1, 2}, {1, 1, 2, 1}, {1, 2, 1, 1},
/// {2, 1, 1, 1}{1, 2, 2}, {2, 1, 2}, {2, 2, 1}
///
/// Expected Time Complexity: O(N)
/// Expected Space Complexity: O(N)
///
/// Constraints:
/// 1 ≤ N ≤ 105
pub fn naive_reach_the_nth_point(n: u32) -> u32 {
    return naive_to_next_point(0, n);
}

pub fn naive_to_next_point(i: u32, n: u32) -> u32 {
    // If we passed the target point, not a solution
    if i > n {
        return 0;
    }

    // If we reached the target point, it's a solution
    if i == n {
        return 1;
    }

    // The number of solution to reach n from i, is the sum of solutions
    // - to reach n from i + 1
    // - to reach n from i + 2
    return naive_to_next_point(i + 1, n) + naive_to_next_point(i + 2, n);
}

pub fn optimized_reach_the_nth_point(n: u32) -> u32 {
    let mut buffer = vec![None::<u32>; n as usize];
    return optimized_to_next_point(0, n, &mut buffer);
}

pub fn optimized_to_next_point(i: u32, n: u32, buffer: &mut Vec<Option<u32>>) -> u32 {
    // If we passed the target point, not a solution
    if i > n {
        return 0;

        // If we reached the target point, it's a solution
    }
    if i == n {
        return 1;
    }

    // Read buffer to avoid exploring the same solution twice
    if buffer[i as usize] != None {
        return buffer[i as usize].unwrap();
    }

    // The number of solution to reach n from i, is the sum of solutions
    // - to reach n from i + 1
    // - to reach n from i + 2
    let result =
        optimized_to_next_point(i + 1, n, buffer) + optimized_to_next_point(i + 2, n, buffer);

    // Update buffer with the newly processed result
    buffer[i as usize] = Some(result);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naive_to_next_point() {
        assert_eq!(naive_to_next_point(0, 1), 1);
        assert_eq!(naive_to_next_point(0, 2), 2);
        assert_eq!(naive_to_next_point(0, 3), 3);
        assert_eq!(naive_to_next_point(0, 4), 5);
        assert_eq!(naive_to_next_point(0, 5), 8);
        assert_eq!(naive_to_next_point(0, 6), 13);
    }

    #[test]
    fn test_optimized_to_next_point() {
        let mut buffer = vec![None::<u32>; 6];
        assert_eq!(optimized_to_next_point(0, 6, &mut buffer), 13);
    }
}
