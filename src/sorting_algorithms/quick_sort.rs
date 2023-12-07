/// ### Principle
///
/// Quicksort is a widely used sorting algorithm based on the divide-and-conquer paradigm.
/// The main steps of the quicksort algorithm are as follows:
///
/// 1. **Partitioning:** A pivot element is chosen from the array,
/// and the other elements are partitioned into two subarrays
/// based on whether they are less than or greater than the pivot.
/// The pivot is then placed in its final sorted position.
///
/// 2. **Recursion:** The partitioning step recursively applies the
/// quicksort algorithm to the subarrays on either side of the pivot
/// until the base case is reached (subarrays with one or zero elements,
/// which are already sorted).
///
/// 3. **Combine:** As the recursion unfolds, the sorted subarrays are
/// combined to produce the overall sorted array.
///
/// Quicksort's efficiency relies on the careful choice of the pivot
/// element, and various strategies exist to mitigate potential
/// performance issues associated with a poorly chosen pivot.
///
/// ### Performances
///
/// - **Time Complexity:** Quicksort has an average-case time complexity
/// of O(n log n), making it one of the most efficient sorting algorithms
/// for a wide range of scenarios. However, its worst-case time complexity
/// is O(n^2) when the pivot selection leads to highly unbalanced partitions.
///
/// - **Space Complexity:** Quicksort is an in-place sorting algorithm,
/// meaning it typically uses only a small, constant amount of additional
/// memory beyond the input array. Its space complexity is O(log n) due
/// to the recursive call stack.
///
/// - **Stability:** Quicksort is not a stable sorting algorithm. The relative
/// order of equal elements is not guaranteed to be preserved during the sorting process.
///
/// - **Adaptability:** Quicksort adapts well to partially sorted arrays
/// and performs efficiently in practice. It is particularly well-suited
/// for large datasets and has a good average-case time complexity.
///
/// - **Notable Advantages:** Quicksort is known for its efficiency,
/// adaptability, and in-place sorting nature. It is often faster
/// in practice than other O(n log n) algorithms due to low constant
/// factors and cache-friendly behavior.
///
/// - **Notable Disadvantages:** The worst-case time complexity of O(n^2)
/// may be a concern in certain scenarios, and careful implementation
/// is required to avoid degenerate cases. Additionally, its lack of
/// stability can be a drawback in applications where maintaining the
/// relative order of equal elements is crucial.
pub fn sort<A>(vec: &mut [A])
where
    A: Ord + Copy,
{
    if vec.len() > 1 {
        let pivot_index = pivot(vec);
        sort(&mut vec[0..pivot_index]);
        sort(&mut vec[pivot_index + 1..]);
    }
}

fn pivot<A>(vec: &mut [A]) -> usize
where
    A: Ord + Copy,
{
    if vec.len() < 2 {
        return 0;
    }

    // Select a pivot value at the head of the vector
    // Init a counter to 0
    let pivot = vec[0];
    let mut counter = 0;

    // Iterate through the vec
    // Swap the values < `pivot` at
    // the beginning of the vec
    // while counting them
    for i in 1..vec.len() {
        let val = vec[i];
        if val > pivot {
            continue;
        } else {
            let temp = vec[counter + 1];
            vec[counter + 1] = val;
            vec[i] = temp;
            counter += 1;
        }
    }

    // Move pivot value at its place
    // (after smaller values, and before bigger values)
    vec[0] = vec[counter];
    vec[counter] = pivot;

    // Return index of the definitive pivot position
    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pivot_should_work() {
        let mut vec = [3, 2, 5, 4, 1, 7, 8, 9];

        let result = pivot(&mut vec);

        assert_eq!(result, 2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 7, 8, 9])
    }

    #[test]
    fn sort_should_work() {
        let mut vec = [0, 3, 5, 1, 12, 4, 8, 2];

        sort(&mut vec);

        assert_eq!(vec, [0, 1, 2, 3, 4, 5, 8, 12]);
    }
}
