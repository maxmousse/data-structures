//! Sorting algorithms
//!
//! | Sorting Algorithm | Description                                                | Time Complexity (Worst Case) | Time Complexity (Best Case) | Time Complexity (Average Case) | Space Complexity | Notable Advantages | Notable Disadvantages |
//! |-------------------|------------------------------------------------------------|-------------------------------|-----------------------------|--------------------------------|-------------------|---------------------|------------------------|
//! | Bubble Sort       | Repeatedly compares and swaps adjacent elements            | O(n^2)                        | O(n)                        | O(n^2)                          | O(1)              | Simple implementation | Inefficient for large datasets |
//! | Selection Sort    | Divides the list into sorted and unsorted regions          | O(n^2)                        | O(n^2)                       | O(n^2)                          | O(1)              | Simple implementation | Inefficient for large datasets |
//! | Insertion Sort    | Builds a sorted list by inserting elements in order        | O(n^2)                        | O(n)                        | O(n^2)                          | O(1)              | Adaptive, efficient for small datasets | Inefficient for large datasets |
//! | Merge Sort        | Divides, sorts, and merges using a divide-and-conquer approach | O(n log n)                  | O(n log n)                   | O(n log n)                      | O(n)              | Stable, consistent performance | Requires additional space for merging |
//! | Quick Sort        | Uses a divide-and-conquer approach with a pivot element    | O(n^2) (rare), O(n log n) avg.| O(n log n)                   | O(n log n)                      | O(log n)          | Efficient for large datasets, in-place partitioning | Not stable, vulnerable to poor pivot choice |
//! | Radix Sort        | Sorts by processing individual digits                      | O(nk), k = digits in largest number | O(nk)                    | O(nk)                          | O(n + k)          | Linear time complexity for certain cases | Requires knowledge of the range of numbers |
pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;
