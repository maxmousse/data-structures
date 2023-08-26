/*

Divide and Conquer

This pattern involves dividing a data set into smaller chunks and then repeating a process with a subset of data.

This pattern can tremendously decrease time complexity

Examples:
- Binary search
- Quick sort
- Merge sort
*/

export function binarySearch(arr: number[], value: number): number {
  let leftPointer = 0;
  let rightPointer = arr.length - 1;

  while (leftPointer <= rightPointer) {
    const middlePointer = Math.floor((leftPointer + rightPointer) / 2);

    if (arr[middlePointer] === value) return middlePointer;

    if (arr[middlePointer] < value) leftPointer = middlePointer + 1;

    if (arr[middlePointer] > value) rightPointer = middlePointer - 1;
  }

  return -1;
}
