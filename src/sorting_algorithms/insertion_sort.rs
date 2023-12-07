/// Sort using insertion sort algorithm
///
/// ### Principle of Insertion Sort Algorithm:
///
/// 1. **Initialization:** The algorithm starts with the assumption that the first element in the array is already sorted. It considers the remaining elements as unsorted.
///
/// 2. **Iteration:** The algorithm iterates through the unsorted portion of the array, taking one element at a time.
///
/// 3. **Insertion:** For each element in the unsorted portion, the algorithm compares it with the elements in the sorted portion, moving from right to left. It finds the correct position for the current element within the sorted portion and inserts it there.
///
/// 4. **Shift:** As the algorithm inserts the current element into its correct position, it may need to shift other elements in the sorted portion to make room for the new element.
///
/// 5. **Repeat:** Steps 2-4 are repeated until all elements are sorted.
///
/// 6. **Completion:** At the end of the process, the entire array is sorted.
///
/// The key idea is to grow the sorted portion of the array by iteratively inserting elements from the unsorted portion into their correct positions. The algorithm is adaptive, meaning it performs well on partially sorted data, and it is also stable, preserving the relative order of equal elements.
///
/// ### Performance Characteristics:
///
/// - **Time Complexity:**
///   - Worst Case: O(n^2) - Occurs when the array is in reverse order, and each element must be compared and shifted.
///   - Best Case: O(n) - This occurs when the array is already sorted, and the algorithm simply inserts each element into its correct position without the need for many comparisons or shifts.
///   - Average Case: O(n^2) - In typical scenarios, the algorithm performs with a quadratic time complexity.
///
/// - **Space Complexity:** O(1) - Insertion Sort is an in-place sorting algorithm, meaning it doesn't require additional memory proportional to the input size.
///
/// - **Stability:** Insertion Sort is stable, meaning that equal elements maintain their relative order in the sorted output.
///
/// - **Adaptability:** It performs well on partially sorted data or small datasets due to its adaptive nature.
///
/// Insertion Sort is straightforward and easy to implement but may not be the best choice for large datasets due to its quadratic time complexity in the worst case.

pub fn sort<T>(vec: &mut [T])
where
    T: Ord + Copy,
{
    for i in 1..vec.len() {
        let curr_val = vec[i];

        for j in (0..=(i - 1)).rev() {
            if vec[j] > curr_val {
                vec[j + 1] = vec[j];
            } else {
                vec[j + 1] = curr_val;
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_should_work() {
        let mut vec = [1, 3, 2, 4, 6, 5];
        let result = [1, 2, 3, 4, 5, 6];

        sort(&mut vec);

        assert_eq!(vec, result);
    }
}
