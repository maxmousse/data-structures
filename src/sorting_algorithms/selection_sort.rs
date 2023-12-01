/// Sort using selection sort algorithm
///
/// ## Principle
///
/// The algorithm divides the input list into two parts:
/// a sorted sublist of items which is built up from left
/// to right at the front (left) of the list and a sublist
/// of the remaining unsorted items that occupy the rest of
/// the list. Initially, the sorted sublist is empty and
/// the unsorted sublist is the entire input list.
/// The algorithm proceeds by finding the smallest
/// (or largest, depending on sorting order) element in the
/// unsorted sublist, exchanging (swapping) it with the
/// leftmost unsorted element (putting it in sorted order),
/// and moving the sublist boundaries one element to the right.
///
/// ## Performances
///
/// Selection sort algorithm has an O(n2) time complexity,
/// which makes it inefficient on large lists, and generally
/// performs worse than the similar insertion sort.
/// Selection sort is noted for its simplicity and
/// has performance advantages over more complicated
/// algorithms in certain situations, particularly
/// where auxiliary memory is limited.
pub fn sort<T>(vec: &mut [T])
where
    T: Ord + Copy,
{
    for i in 0..vec.len() {
        let mut min_index = i;

        for j in i..vec.len() {
            if vec[min_index] > vec[j] {
                min_index = j;
            }
        }

        // Swap the min value, only if its not
        // already sorted =)
        if i != min_index {
            let temp = vec[i];
            vec[i] = vec[min_index];
            vec[min_index] = temp;
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
