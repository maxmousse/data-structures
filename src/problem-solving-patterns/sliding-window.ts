/*

Sliding window pattern

Involves creating a window (a sub array) and moving it
throw the array to explore

The window could move, increase, decrease, depending of the goal

Useful to find a continuous subset matching a condition

Examples: 
- find the longest substring of a string that contains only
unique characters

- In a number[], find the sum of n consecutive elements that gives the maximum
value
*/

export function maxSubArraySum(
  arr: number[],
  windowSize: number
): number | null {
  if (windowSize > arr.length) return null;

  let currentSubSum: number = 0;
  let maxSubSum: number;

  // Calculate the sum for the initial window
  for (let i = 0; i < windowSize; i++) {
    currentSubSum += arr[i];
  }

  // Init maxSubSum to currentSubSum
  maxSubSum = currentSubSum;

  // Iterate throw the array, and rather than recalculate
  // the whole some for each window, add the new value and remove the old
  // one from the current sum
  for (let i = 1; i < arr.length - windowSize; i++) {
    currentSubSum = currentSubSum - arr[i - 1] + arr[i + windowSize];
    maxSubSum = Math.max(currentSubSum, maxSubSum);
  }

  return maxSubSum;
}
