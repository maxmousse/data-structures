/// Merge sort algorithm
///
/// ## Principle
///
/// Merge Sort is a divide-and-conquer algorithm that efficiently
/// sorts an array or list by recursively dividing it into smaller halves,
/// sorting each half, and then merging the sorted halves back together.
/// The key steps are as follows:
///
/// 1. **Divide:** The array is divided into two halves, and the process is
/// recursively applied to each half until the base case is reached (
/// an array with one or zero elements, which is inherently sorted).
///
/// 2. **Conquer:** Each divided subarray is sorted independently using
/// the same merge sort algorithm.
///
/// 3. **Merge:** The sorted subarrays are then merged back together in a
/// way that combines them into a single sorted array. This merging
/// step is the core of the algorithm, involving comparing and arranging
/// elements from the two halves.
///
/// 4. **Repeat:** Steps 1-3 are repeated until the entire array is sorted.
///
/// The merge sort algorithm is stable, meaning that it preserves the relative
///  order of equal elements during the sorting process.
///
/// ## Performances
///
/// - **Time Complexity:** The time complexity of merge sort is O(n log n) in the worst,
/// best, and average cases. This makes it one of the most consistently efficient
/// sorting algorithms, suitable for large datasets.
///
/// - **Space Complexity:** Merge sort requires additional memory proportional to
/// the size of the input array due to the temporary storage needed during the
/// merging process. Therefore, its space complexity is O(n).
///
/// - **Stability:** Merge sort is a stable sorting algorithm, meaning that it
/// maintains the relative order of equal elements in the sorted output.
///
/// - **Adaptability:** Merge sort performs consistently well regardless of the
/// initial order of the elements, making it suitable for a wide range of scenarios.
/// Its efficiency is particularly notable for linked lists, as it does not rely
/// on random access to elements.
///
/// - **Notable Advantages:** Merge sort's main advantages include
/// its stable nature, consistent performance across different
/// input scenarios, and suitability for large datasets.
///
/// - **Notable Disadvantages:** The main disadvantage of merge sort is
/// its space complexity, as it requires additional memory for the
/// temporary arrays during the merging step. This can be a concern
/// for large datasets with limited available memory.
///
pub fn sort<A>(vec: &[A]) -> Vec<A>
where
    A: Ord + Copy,
{
    // Recursion base condition
    // when we reach the bottom of the source vector
    // (once it's splitted) in a vec of len of 0 or 1)
    if vec.len() <= 1 {
        return vec.to_vec();
    }

    // Split vec in 2
    // Then recursively split, sort and merge
    let middle = vec.len() / 2;
    let left = sort(&vec[0..middle]);
    let right = sort(&vec[middle..]);
    naive_merge_sorted_vecs(&left, &right)
}

// merge_sorted_vecs as its not optimized
///
/// It copies the vectors rather than editing them in place
/// There is a method to avoid copy, but it requires `unsafe` mode
fn naive_merge_sorted_vecs<A>(vec_1: &[A], vec_2: &[A]) -> Vec<A>
where
    A: Ord + Copy,
{
    let mut result = vec![];
    let mut curs_1 = 0;
    let mut curs_2 = 0;

    // Iterate through both sorted array, and push the values
    // `result` while keeping the order, until one of the source
    // array is fully, merged
    while curs_1 < vec_1.len() && curs_2 < vec_2.len() {
        if vec_1[curs_1] < vec_2[curs_2] {
            result.push(vec_1[curs_1]);
            curs_1 += 1;
        } else {
            result.push(vec_2[curs_2]);
            curs_2 += 1;
        }
    }

    // Then, add the rest of the remaining array
    // At this step, one array is empty, we push what
    // remains in the other into result
    result.extend(&vec_1[curs_1..]);
    result.extend(&vec_2[curs_2..]);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn naive_merge_sorted_vecs_should_work() {
        assert_eq!(naive_merge_sorted_vecs::<i32>(&[], &[]), vec![]);
        assert_eq!(
            naive_merge_sorted_vecs::<i32>(&[1, 2, 8], &[0, 3, 4, 12]),
            vec![0, 1, 2, 3, 4, 8, 12]
        );
    }

    #[test]
    fn naive_sort_should_work() {
        assert_eq!(sort::<i32>(&Vec::new()), vec![]);
        assert_eq!(sort(&vec![6, 3, 1, 2, 5, 4]), vec![1, 2, 3, 4, 5, 6]);
    }
}
