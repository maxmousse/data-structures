//! Gold mine problem (https://www.geeksforgeeks.org/problems/gold-mine-problem2608/1)
//!
//! Given a gold mine called M of (n x m) dimensions. Each field in this mine contains a positive integer
//! which is the amount of gold in tons. Initially the miner can start from any row
//! in the first column. From a given cell, the miner can move:
//!
//! - to the cell diagonally up towards the right
//! - to the right
//! - to the cell diagonally down towards the right
//!
//! Find out maximum amount of gold which he can collect until he can no longer move.
//!
//! Example 1:
//!
//! Input: n = 3, m = 3
//! M = {{1, 3, 3},
//!  {2, 1, 4},
//!  {0, 6, 4}};
//! Output: 12
//! Explanation:
//! The path is {(1,0) -> (2,1) -> (2,2)}.
//!
//!
//! Example 2:
//!
//! Input: n = 4, m = 4
//! M = {{1, 3, 1, 5},
//!  {2, 2, 4, 1},
//!  {5, 0, 2, 3},
//!  {0, 6, 1, 2}};
//! Output: 16
//! Explanation:
//! The path is {(2,0) -> (3,1) -> (2,2)
//! -> (2,3)} or {(2,0) -> (1,1) -> (1,2)
//! -> (0,3)}.
//!
//! Your Task:
//! You do not need to read input or print anything. Your task is to complete the function maxGold() which takes the values n, m and the mine represented as a 2D array of positive integeres M as input parameters and returns the maximum amount of gold that can be collected.
//!
//! Expected Time Complexity: O(n*m)
//! Expected Auxiliary Space: O(n*m)
//!
//! Constraints:
//! 1 ≤ n, m ≤ 500
//! 0 ≤ M[i][j] ≤ 100

use std::cmp::max;

/// Find the "longest path" (the path with most gold), by exploring all possible paths
/// recursively from a start node
///
/// NOTE: It wastes a lot of time by exploring the same branches several times and can be optimized by the
/// memoized version !
///
/// To find the longest path of a matrix, it should be called on all 'starting nodes'
///
/// Time complexity: O(3N*M)
/// Auxiliary Space: O(N*M)
pub fn collect_gold_greedy(
    gold_map: &Vec<Vec<u32>>,
    x: isize,
    y: usize,
    n: usize,
    m: usize,
    call_count: &mut u32, // just for testing purpose to compare greedy vs memoized
) -> u32 {
    // increment call_count
    *call_count += 1;

    // Base condition: if out of M bound, return 0
    if x < 0 || x as usize == n || y == m {
        return 0;
    }

    // Get amount of gold of next nodes
    let upper_right = collect_gold_greedy(gold_map, x - 1, y + 1, n, m, call_count);
    let right = collect_gold_greedy(gold_map, x, y + 1, n, m, call_count);
    let lower_right = collect_gold_greedy(gold_map, x + 1, y + 1, n, m, call_count);

    // Determine the maximum gold at the current node
    return gold_map[x as usize][y] + max(upper_right, max(right, lower_right));
}

/// This version is optimized compared to the greedy version
/// It uses a buffer to keep track of the max_gold that is harvestable
/// at each node, avoiding
pub fn collect_gold_memoized(
    gold_map: &Vec<Vec<u32>>,
    buffer: &mut Vec<Vec<Option<u32>>>,
    x: isize,
    y: usize,
    n: usize,
    m: usize,
    call_count: &mut u32, // just for testing purpose to compare greedy vs memoized
) -> u32 {
    // increment call_count
    *call_count += 1;

    // Base condition: if out of M bound, return 0
    if x < 0 || x as usize == n || y == m {
        return 0;
    }

    // If max amount of gold at the current node has been determined already
    // use the value in the buffer
    if buffer[x as usize][y] != None {
        return buffer[x as usize][y].unwrap();
    }

    // Get amount of gold of next nodes
    let upper_right = collect_gold_memoized(gold_map, buffer, x - 1, y + 1, n, m, call_count);
    let right = collect_gold_memoized(gold_map, buffer, x, y + 1, n, m, call_count);
    let lower_right = collect_gold_memoized(gold_map, buffer, x + 1, y + 1, n, m, call_count);

    // Determine the maximum gold at the current node
    let max_gold = gold_map[x as usize][y] + max(upper_right, max(right, lower_right));

    // Update the buffer
    buffer[x as usize][y] = Some(max_gold);

    max_gold
}

pub fn get_max_gold_greedy(gold_map: &Vec<Vec<u32>>, call_count: &mut u32) -> u32 {
    let n = gold_map.len();
    let m = gold_map[0].len();

    let mut max_gold = 0;

    // Iter through all starting nodes
    for i in 0..gold_map.len() {
        max_gold = max(
            max_gold,
            collect_gold_greedy(gold_map, i as isize, 0, n, m, call_count),
        );
    }

    max_gold
}

pub fn get_max_gold_memoized(gold_map: &Vec<Vec<u32>>, call_count: &mut u32) -> u32 {
    let n = gold_map.len();
    let m = gold_map[0].len();

    let mut max_gold = 0;

    // Init buffer with None
    let mut buffer = vec![vec![None::<u32>; m]; n];

    // Iter through all starting nodes
    for i in 0..gold_map.len() {
        max_gold = max(
            max_gold,
            collect_gold_memoized(gold_map, &mut buffer, i as isize, 0, n, m, call_count),
        );
    }

    max_gold
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_gold_greedy() {
        let gold_map = vec![vec![1, 4, 3], vec![5, 1, 2], vec![8, 1, 1]];
        // 1, 4, 3
        // 5, 1, 2
        // 8, 1, 1
        let n = gold_map.len();
        let m = gold_map[0].len();
        let mut call_count = 0;

        assert_eq!(
            collect_gold_greedy(&gold_map, 0, 0, n, m, &mut call_count),
            8
        );
        assert_eq!(
            collect_gold_greedy(&gold_map, 1, 0, n, m, &mut call_count),
            12
        );
        assert_eq!(
            collect_gold_greedy(&gold_map, 2, 0, n, m, &mut call_count),
            12
        );
    }

    #[test]
    fn test_get_max_gold_greedy() {
        let gold_map = vec![
            vec![1, 4, 3, 12, 6],
            vec![5, 1, 4, 12, 2],
            vec![8, 1, 1, 7, 10],
            vec![2, 1, 5, 3, 10],
            vec![2, 1, 5, 3, 10],
        ];

        let mut call_count = 0;
        assert_eq!(get_max_gold_greedy(&gold_map, &mut call_count), 35);
        println!("CALL COUNT: {}", call_count);
    }

    #[test]
    fn test_collect_gold_memoized() {
        let gold_map = vec![vec![1, 4, 3], vec![5, 1, 2], vec![8, 1, 1]];
        // 1, 4, 3
        // 5, 1, 2
        // 8, 1, 1

        let n = gold_map.len();
        let m = gold_map[0].len();
        let mut buffer = vec![vec![None::<u32>; m]; n];
        let mut call_count = 0;

        assert_eq!(
            collect_gold_memoized(&gold_map, &mut buffer, 0, 0, n, m, &mut call_count),
            8
        );
        assert_eq!(
            collect_gold_memoized(&gold_map, &mut buffer, 1, 0, n, m, &mut call_count),
            12
        );
        assert_eq!(
            collect_gold_memoized(&gold_map, &mut buffer, 2, 0, n, m, &mut call_count),
            12
        );
    }

    #[test]
    fn test_get_max_gold_memoized() {
        let gold_map = vec![
            vec![1, 4, 3, 12, 6],
            vec![5, 1, 4, 12, 2],
            vec![8, 1, 1, 7, 10],
            vec![2, 1, 5, 3, 10],
            vec![2, 1, 5, 3, 10],
        ];
        let mut call_count = 0;

        assert_eq!(get_max_gold_memoized(&gold_map, &mut call_count), 35);
        println!("CALL COUNT: {}", call_count);
    }
}
