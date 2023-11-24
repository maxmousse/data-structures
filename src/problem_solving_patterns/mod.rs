/// Frequency counter pattern
///
/// This pattern uses objects or sets to collect values/frequencies of values
/// This can often avoid the need for nested loops or O(n^2) operations with arrays/strings
pub mod frequency_counter;

/// Multiple pointers
///
/// Creating pointers or values that correspond to an index position
/// and move towards the beginning, end or middle based on certain
/// conditions
///
/// Very efficient for solving problems with minimal space
/// complexity
///
/// Use cases:
/// - find pairs in sorted array
/// - find unique values in sorted array (also I prefer using a frequency map for this one)
pub mod multiple_pointers;

/// Divide and Conquer
///
/// This pattern involves dividing a data set into smaller chunks and then repeating a process with a subset of data.
/// This pattern can tremendously decrease time complexity
/// Examples:
/// - Binary search
/// - Quick sort
/// - Merge sort
pub mod divide_and_conquer;

/// Sliding window pattern
///
/// Involves creating a window (a sub array) and moving it
/// throw the array to explore
/// The window could move, increase, decrease, depending of the goal
/// Useful to find a continuous subset matching a condition
///
/// Examples:
/// - find the longest substring of a string that contains only
/// unique characters
/// - In a number[], find the sum of n consecutive elements that
/// gives the maximum value
pub mod sliding_window;
