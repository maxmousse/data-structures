/// Radix sort algorithm
///
/// ## Principle of Radix Sort Algorithm:
///
/// Radix Sort is a non-comparative integer sorting algorithm that processes
/// individual digits of numbers. The main steps of the radix sort algorithm
/// are as follows:
///
/// 1. **Digit Extraction:** Starting from the least significant digit to
/// the most significant digit, the numbers are processed, and elements
/// are grouped based on the value of the current digit.
///
/// 2. **Counting Sort for Each Digit:** For each digit place, a
/// stable sorting algorithm (commonly counting sort) is
/// used to sort the elements based on that digit.
///
/// 3. **Repeat for All Digits:** Steps 1-2 are repeated for
/// each digit place until the entire array is sorted.
///
/// The effectiveness of radix sort relies on the stable
/// sorting of each digit place and the ability to maintain
/// the relative order of equal elements during each pass.
///
/// ## Performances of Radix Sort:
///
/// - **Time Complexity:** Radix sort has a time complexity of
/// O(nk), where n is the number of elements and k is the number
/// of digits in the largest number. Its linear time complexity
/// for each digit place contributes to its overall efficiency.
///
/// - **Space Complexity:** Radix sort is an in-place sorting algorithm,
/// meaning it uses a constant amount of additional memory beyond the
/// input array. Its space complexity is O(n + k).
///
/// - **Stability:** Radix sort is a stable sorting algorithm, meaning
/// that the relative order of equal elements is preserved during
/// the sorting process.
///
/// - **Adaptability:** Radix sort is well-suited for sorting integers
/// or fixed-size keys, and its performance is not affected by the
/// initial order of the elements. It may not be as efficient for
/// variable-size keys or floating-point numbers.
///
/// - **Notable Advantages:** Radix sort is efficient for large datasets of
/// integers, particularly when the range of key values is limited. Its
/// linear time complexity for each digit place makes it suitable for
/// scenarios where other algorithms might be less efficient.
///
/// - **Notable Disadvantages:** Radix sort is specific to integer keys and
/// may not be suitable for all types of data. It may also require additional
/// memory for counting sort during each pass, and its performance can
/// degrade if the key values have a large range.
///
pub fn sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    // Find the maximum number to know the number of digits
    let max_num = *arr.iter().max().unwrap();
    let num_digits = ((max_num as f64).log10() + 1.0) as usize;

    // Perform counting sort for each digit place
    for i in 0..num_digits {
        let mut count = vec![0; 10];
        let mut output = vec![0; arr.len()];

        // Count occurrences of each digit
        for &num in arr.iter() {
            count[get_digit(num, i)] += 1;
        }

        // Update count to store the position of each digit in the output
        for j in 1..10 {
            count[j] += count[j - 1];
        }

        // Build the output array using count to determine positions
        for &num in arr.iter().rev() {
            let digit = get_digit(num, i);
            output[count[digit] - 1] = num;
            count[digit] -= 1;
        }

        // Copy the sorted output back to the original array
        arr.copy_from_slice(&output);
    }
}

/// Helper function to get the digit at a specific position
fn get_digit(num: usize, position: usize) -> usize {
    (num / 10usize.pow(position as u32)) % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit() {
        assert_eq!(get_digit(123, 0), 3);
        assert_eq!(get_digit(123, 1), 2);
        assert_eq!(get_digit(123, 2), 1);
        assert_eq!(get_digit(789, 0), 9);
        assert_eq!(get_digit(789, 1), 8);
        assert_eq!(get_digit(789, 2), 7);
    }

    #[test]
    fn test_sort() {
        let mut data = vec![170, 45, 75, 90, 802, 24, 2, 66];
        sort(&mut data);
        assert_eq!(data, vec![2, 24, 45, 66, 75, 90, 170, 802]);

        let mut empty_data: Vec<usize> = Vec::new();
        sort(&mut empty_data);
        assert_eq!(empty_data, Vec::<usize>::new());

        let mut single_element_data = vec![42];
        sort(&mut single_element_data);
        assert_eq!(single_element_data, vec![42]);

        let mut duplicate_data = vec![9, 9, 9, 9, 9];
        sort(&mut duplicate_data);
        assert_eq!(duplicate_data, vec![9, 9, 9, 9, 9]);
    }
}
