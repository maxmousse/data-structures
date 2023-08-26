/*
    
Frequency counter pattern

This pattern uses objects or sets to collect values/frequencies of values
This can often avoid the need for nested loops or O(n^2) operations with arrays/strings

 */

/**
 *  Example 1 - Group by
 */
export function groupBy<T extends Record<string, string>>(
  arr: T[],
  key: keyof T
): Record<string, T[]> {
  // Iterate through the array and group by the key
  return arr.reduce((acc, item) => {
    const group = item[key];

    return {
      ...acc,
      [group]: acc[group] ? [...acc[group], item] : [item],
    };
  }, {} as Record<string, T[]>);
}

/**
 *  Example 2 - Anagram
 */
export function isAnagram(str1: string, str2: string): boolean {
  // Early fail, as if string are of different length
  // They cannot be anagrams
  if (str1.length !== str2.length) return false;

  // Build frequency map of str1
  const str1FrequencyMap = Array.from(str1).reduce(
    (frequencyMap, char) => ({
      ...frequencyMap,
      [char]: frequencyMap[char] ? frequencyMap.char + 1 : 1,
    }),
    {} as Record<string, number>
  );

  // Build frequency map of str2
  const str2FrequencyMap = Array.from(str2).reduce(
    (frequencyMap, char) => ({
      ...frequencyMap,
      [char]: frequencyMap[char] ? frequencyMap.char + 1 : 1,
    }),
    {} as Record<string, number>
  );

  // Early fail, as if frequency maps does not contain the same number of charters
  // string cannot be anagrams
  if (Object.keys(str1FrequencyMap) !== Object.keys(str2FrequencyMap))
    return false;

  // Compare frequency maps
  Object.entries(str1FrequencyMap).forEach(([key, frequency]) => {
    if (frequency !== str2FrequencyMap[key]) return false;
  });

  return true;
}

/**
 *  Example 3: Optimized anagram with 'unstacking' for map frequency comparison
 */
function isAnagramOptimized(first: string, second: string) {
  if (first.length !== second.length) {
    return false;
  }

  const lookup: Record<string, number> = {};

  for (let i = 0; i < first.length; i++) {
    let letter = first[i];
    // if letter exists, increment, otherwise set to 1
    lookup[letter] ? (lookup[letter] += 1) : (lookup[letter] = 1);
  }

  for (let i = 0; i < second.length; i++) {
    let letter = second[i];
    // can't find letter or letter is zero then it's not an anagram
    if (!lookup[letter]) {
      return false;
    } else {
      // IMPORTANT: subtract frequency of 'second' to frequency of 'first'
      lookup[letter] -= 1;
    }
  }

  return true;
}
