/*
Multiple pointers

Creating pointers or values that corespond to an index position
and move towards the beginning, end or middle based on certain
conditions

Very efficient for solving problems with minimal space
complexity

Use cases:
- find pairs in sorted array
- find unique values in sorted array (also I prefer using a frequency map for this one)
*/

/**
 * Example 1: find the first pair for which the sum
 * result is zero
 */
function sumZero(arr: number[]): [number, number] | null {
  let leftPointer = 0;
  let rightPointer = 1;

  // Iterates through the array until the two pointers meet
  while (leftPointer < rightPointer) {
    // If there is a pair, return
    if (arr[leftPointer] + arr[rightPointer] === 0)
      return [arr[leftPointer], arr[rightPointer]];

    // If sum is positive, move rightPointer to a lower value
    if (arr[leftPointer] + arr[rightPointer] > 0) rightPointer--;

    // If sum is negative move leftPointer to a bigger value
    if (arr[leftPointer] + arr[rightPointer] < 0) leftPointer++;
  }

  // Return null if there is no pair
  return null;
}

/*
 * Example 2: count unique values
 * Note : this function uses 'i' counter as the counter
 * of unique values
 * I think it could be done with a reduce with almost the
 * same efficiency
 */
function countUniqueValues(arr: number[]) {
  // If arr is empty, return 0
  // This short circuit is required as the return
  // value would be '1' without it for an empty array
  if (arr.length === 0) return 0;

  // init i at the beginning of the array
  let i = 0;

  // Init j at the second value of the array
  for (var j = 1; j < arr.length; j++) {
    // When i and j different
    if (arr[i] !== arr[j]) {
      // move i forward and register the new
      // unique value found at j
      i++;
      arr[i] = arr[j];
    }
  }
  // i + 1 = number of unique values found in the array
  return i + 1;
}
