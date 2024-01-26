/// Painting the fence
/// Given a fence with n posts and k colors, find out the number of ways of painting
/// the fence so that not more than two consecutive posts have the same colors.
/// Since the answer can be large return it modulo 109 + 7.
///
/// Example 1:
///
/// Input:
/// n = 3
/// k = 2
/// Output: 6
/// Explanation:
/// We have following possible combinations:
///
///  
///
/// Example 2:
///
/// Input:
/// n = 2
/// k = 4
/// Output: 16
/// Explanation:
/// After coloring first post with
/// 4 possible combinations, you can still color
/// next posts with all 4 colors. Total possible
/// combinations will be 4x4=16
///
/// Expected Time Complexity: O(N).
/// Expected Auxiliary Space: O(N).
///
/// Constraints:
/// 1 ≤ N ≤ 105
/// 1 ≤ K ≤ 105
pub fn paint_the_fence(posts: u32, colors: u32) -> u32 {
    // If 0 post, no combinations
    if colors == 0 {
        return 0;
    }

    match posts {
        // If 0 post, no combinations
        0 => 0,
        // If 1 post, `colors` combinations
        1 => colors,
        // If 2 posts, `colors * colors` combinations
        2 => colors * colors,
        _ => {
            // Explanations
            //
            // For a binary string of size n, let say we have:
            // - combinations_without_consecutive_colors(n) combinations that do not end with 2 consecutive posts of the same color
            // - combinations_with_consecutive_colors(n) combinations that do end with 2 consecutive posts of the same color
            //
            // Example: n = 2, colors = 3
            // combinations_without_consecutive_colors(2) = 3 (bb, rr, gg)
            // combinations_with_consecutive_colors(2) = 6 (br, bg, rb, rg, gb, gr)
            //
            // For a string of size n + 1 ( <=> to add a post in front of the allowed combinations for a string of size n)
            // combinations_with_consecutive_colors(n+1) = combinations_without_consecutive_colors(n)
            //   (each combinations_without_consecutive_colors(n) generate exactly one with consecutive,
            //   by adding the same color at the end. combinations_with_consecutive_colors cannot generate combinations_with_consecutive_colors by adding a post)
            // combinations_without_consecutive_colors(n+1) = (colors - 1) * (total_allowed_combinations(n)
            let mut i = 2;
            let mut combinations_without_consecutive_colors = colors * (colors - 1);
            let mut combinations_with_consecutive_colors = colors;
            let mut total_allowed_combinations =
                combinations_with_consecutive_colors + combinations_without_consecutive_colors;

            while i < posts {
                combinations_with_consecutive_colors = combinations_without_consecutive_colors;
                combinations_without_consecutive_colors = (colors - 1) * total_allowed_combinations;
                total_allowed_combinations =
                    combinations_with_consecutive_colors + combinations_without_consecutive_colors;
                i += 1;
            }

            total_allowed_combinations
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_paint_the_fence() {
        // If no color, no combination
        assert_eq!(paint_the_fence(3, 0), 0);

        // If no post, no combination
        assert_eq!(paint_the_fence(0, 3), 0);

        // If one post, combination == colors
        assert_eq!(paint_the_fence(1, 5), 5);

        // If 2 posts, combination == colors * colors
        assert_eq!(paint_the_fence(2, 5), 25);

        // Check real values
        assert_eq!(paint_the_fence(3, 5), 120);
        assert_eq!(paint_the_fence(4, 5), 580);
        assert_eq!(paint_the_fence(5, 5), 2800);
        assert_eq!(paint_the_fence(6, 5), 13520);
    }
}
