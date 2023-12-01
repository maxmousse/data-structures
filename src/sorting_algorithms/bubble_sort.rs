/// Sort using bubble sort algorithm
///
/// ## Principle
///
/// Bubble sort algorithm is a simple sorting algorithm that repeatedly steps through
/// the input list element by element, comparing the current element
/// with the one after it, swapping their values if needed. These passes
/// through the list are repeated until no swaps had to be performed
/// during a pass, meaning that the list has become fully sorted.
/// The algorithm, which is a comparison sort, is named for the
/// way the larger elements "bubble" up to the top of the list
///
/// ## Performances
///
/// Bubble sort has a worst-case and average complexity of O(n^{2}), where n n is
/// the number of items being sorted. Most practical sorting algorithms have
/// substantially better worst-case or average complexity, often O(n\log n).
/// Even other O(n^{2}) sorting algorithms, such as insertion sort, generally
/// run faster than bubble sort, and are no more complex. For this reason,
/// bubble sort is rarely used in practice.
///
/// Like insertion sort, bubble sort is adaptive, giving it an advantage over
/// algorithms like quicksort. This means that it may outperform those algorithms
/// in cases where the list is already mostly sorted (having a small number of inversions),
/// despite the fact that it has worse average-case time complexity. For example,
/// bubble sort is O ( n ) O(n) on a list that is already sorted, while quicksort
/// would still perform its entire O(n\log n) sorting process.
///
/// While any sorting algorithm can be made O ( n ) O(n) on a presorted list simply by
/// checking the list before the algorithm runs, improved performance on almost-sorted
/// lists is harder to replicate
pub fn sort<T>(vec: &mut [T])
where
    T: Ord + Copy,
{
    for i in (0..vec.len()).rev() {
        // Allow to exit as soon as the vec is sorted
        let mut swapped = false;

        for j in 0..i {
            if vec[j] > vec[j + 1] {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
                // Denote than a swapped took place which means the vector
                // is not fully ordered yet
                swapped = true;
            }
        }

        if swapped == false {
            break;
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
