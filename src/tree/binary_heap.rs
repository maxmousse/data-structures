#[derive(Debug)]

/// Binary heap
///
/// A binary heap is a specialized tree-based data structure that forms a complete binary tree.
/// In a binary heap, every level of the tree is filled, except possibly the last level,
/// which is filled from left to right. The defining property of a binary heap is the heap property,
/// which can be either a min-heap or a max-heap. In a min-heap, the value of each node is less than or
/// equal to the values of its children, ensuring that the smallest element is at the root. Conversely,
/// in a max-heap, the value of each node is greater than or equal to the values of its children,
/// making the largest element the root. This hierarchical structure, combined with efficient insertion and extraction operations,
/// makes binary heaps particularly useful for implementing priority queues and heap-sort algorithms.
/// The inherent balance and order of binary heaps contribute to their suitability for applications where quick
/// access to the smallest or largest element is essential.
///
/// ## Array representation
///
/// A binary heap can be efficiently stored in an array, leveraging the properties of a complete binary tree.
/// The array representation of a binary heap has several advantages, including simplicity and efficient use of memory.
/// Here's how a binary heap can be stored in an array:
///
/// 1. **Complete Binary Tree Property:**
///    - Since a binary heap is a complete binary tree, it can be stored in an array by using the natural ordering of the tree nodes.
///    - Nodes are filled in a left-to-right fashion across each level.
///
/// 2. **Indexing:**
///    - Starting from index 1, each index in the array corresponds to a node in the binary heap.
///    - For any node at index `i`:
///      - Its left child is at index `2 * i`.
///      - Its right child is at index `2 * i + 1`.
///      - Its parent is at index `i / 2` (integer division).
///
/// 3. **Root at Index 1:**
///    - To simplify the calculations and make the indexing more intuitive, it's common to start the heap at index 1 instead of 0.
///    - The root of the heap is at index 1.
///
/// ### Example:
///
/// Consider the following binary heap:
///    10
///  /  \
///  20   15
/// / \   /
/// 30 25 40
///
/// In array representation (starting from index 1), it would be: [10, 20, 15, 30, 25, 40]
///
/// - Index 1: Root (10)
/// - Index 2: Left child of the root (20)
/// - Index 3: Right child of the root (15)
/// - Index 4: Left child of the node at index 2 (30)
/// - Index 5: Right child of the node at index 2 (25)
/// - Index 6: Left child of the node at index 3 (40)
///
/// ### Benefits:
///
/// 1. **Space Efficiency:**
///    - The array representation uses less memory compared to linked structures, as it avoids the overhead of storing explicit pointers.
///
/// 2. **Index Calculations:**
///    - Indexing calculations are straightforward, enabling efficient navigation to parent, left child, and right child nodes.
///
/// 3. **Cache Locality:**
///    - Elements in the array are stored contiguously, promoting better cache locality, which can enhance performance.
///
/// The array representation of a binary heap facilitates efficient storage, indexing, and traversal, making it a preferred choice for many applications.

pub struct MaxBinaryHeap<T: Copy + PartialOrd> {
    pub values: Vec<T>,
}

impl<T: Copy + PartialOrd> MaxBinaryHeap<T> {
    /// Create a new instance of MaxBinaryHeap
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Insert a new value in the binary heap
    ///
    /// To maintain the heap property, the new value is inserted at the end of the heap,
    /// then, its bubbled up until it reaches a position that respect the heap order
    pub fn insert(&mut self, value: T) {
        // Push the value at the end of the heap
        self.values.push(value);

        // Determine the index of the parent node
        let mut value_index = self.values.len() - 1;
        let mut parent_index_option = self.get_parent_index_of(value_index);

        // While the inserted value is smaller than its parent node, bubble it up
        while let Some(parent_index) = parent_index_option {
            if self.values[value_index] < self.values[parent_index] {
                break;
            }

            // Swap values
            let temp = self.values[parent_index];
            self.values[parent_index] = self.values[value_index];
            self.values[value_index] = temp;

            // Update indexes for next iteration
            value_index = parent_index;
            parent_index_option = self.get_parent_index_of(value_index);
        }
    }

    /// Extract the to value of the heap
    ///
    /// To maintain the heap property, the last value of the heap is put at the root,
    /// then it is bubbled down until it reaches a position tha respect the heap order
    pub fn extract(&mut self) -> Option<T> {
        // Pop the last item
        let last = self.values.pop();

        // if its none ( <=> the array was empty)
        // or if values is empty after the pop (<=> its length was 1)
        // return last
        if last.is_none() || self.values.is_empty() {
            return last;
        }

        // Extract first, and replace it with last
        let first = self.values[0];
        self.values[0] = last.unwrap();

        // Bubble down first until the heap order is valid
        let mut value_index = 0;
        let mut index_of_greater_child_option = self.get_index_of_greater_child(value_index);

        while let Some(index_of_greater_child) = index_of_greater_child_option {
            if self.values[value_index] > self.values[index_of_greater_child] {
                break;
            }

            // Swap values
            let temp = self.values[index_of_greater_child];
            self.values[index_of_greater_child] = self.values[value_index];
            self.values[value_index] = temp;

            // Update indexes for next iteration
            value_index = index_of_greater_child;
            index_of_greater_child_option = self.get_index_of_greater_child(value_index);
        }

        // Return first
        Some(first)
    }

    pub fn get_parent_index_of(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    pub fn get_index_of_first_child(&self, index: usize) -> Option<usize> {
        let result = index * 2 + 1;

        if result < self.values.len() {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_index_of_second_child(&self, index: usize) -> Option<usize> {
        let result = index * 2 + 2;

        if result < self.values.len() {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_index_of_greater_child(&self, index: usize) -> Option<usize> {
        let index_of_first_child = self.get_index_of_first_child(index);
        let index_of_second_child = self.get_index_of_second_child(index);

        if index_of_second_child.is_none() {
            index_of_first_child
        } else {
            if self.values[index_of_first_child.unwrap()]
                > self.values[index_of_second_child.unwrap()]
            {
                index_of_first_child
            } else {
                index_of_second_child
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut heap: MaxBinaryHeap<u32> = MaxBinaryHeap::new();

        assert_eq!(heap.values, Vec::new());

        heap.insert(10);
        assert_eq!(heap.values, vec![10]);

        heap.insert(8);
        assert_eq!(heap.values, vec![10, 8]);

        heap.insert(12);
        assert_eq!(heap.values, vec![12, 8, 10]);

        heap.insert(9);
        assert_eq!(heap.values, vec![12, 9, 10, 8]);

        heap.insert(11);
        assert_eq!(heap.values, vec![12, 11, 10, 8, 9]);

        heap.insert(4);
        assert_eq!(heap.values, vec![12, 11, 10, 8, 9, 4]);

        heap.insert(20);
        assert_eq!(heap.values, vec![20, 11, 12, 8, 9, 4, 10]);

        heap.insert(20);
        assert_eq!(heap.values, vec![20, 20, 12, 11, 9, 4, 10, 8]);
    }

    #[test]
    fn test_extract() {
        let mut heap: MaxBinaryHeap<u32> = MaxBinaryHeap::new();
        heap.insert(20);
        heap.insert(20);
        heap.insert(12);
        heap.insert(11);
        heap.insert(10);
        heap.insert(9);
        heap.insert(8);
        heap.insert(4);

        assert_eq!(heap.extract(), Some(20));
        assert_eq!(heap.extract(), Some(20));
        assert_eq!(heap.extract(), Some(12));
        assert_eq!(heap.extract(), Some(11));
        assert_eq!(heap.extract(), Some(10));
        assert_eq!(heap.extract(), Some(9));
        assert_eq!(heap.extract(), Some(8));
        assert_eq!(heap.extract(), Some(4));
        assert_eq!(heap.extract(), None);
    }
}
