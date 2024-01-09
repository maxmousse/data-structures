use std::collections::VecDeque;

/// A node of a binary search tree
///
/// It allows at most 2 children (binary tree)
#[derive(Debug)]
struct Node<T: Copy + PartialOrd> {
    elem: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// Binary search tree implementation
///
/// In computer science, a binary search tree (BST), is a rooted binary tree data structure with the key of each internal
/// node being greater than all the keys in the respective node's left subtree and less than the ones in its right subtree.
///  The time complexity of operations on the binary search tree is linear with respect to the height of the tree.
///
/// Binary search trees allow binary search for fast lookup, addition, and removal of data items. Since the nodes in a
/// BST are laid out so that each comparison skips about half of the remaining tree, the lookup performance is proportional
/// to that of binary logarithm. BSTs were devised in the 1960s for the problem of efficient storage of labeled data and are
/// attributed to Conway Berners-Lee and David Wheeler.
///
/// The performance of a binary search tree is dependent on the order of insertion of the nodes into the tree since arbitrary
/// insertions may lead to degeneracy; several variations of the binary search tree can be built with guaranteed worst-case performance.
/// The basic operations include: search, traversal, insert and delete. BSTs with guaranteed worst-case complexities perform better
/// than an unsorted array, which would require linear search time.
///
/// The complexity analysis of BST shows that, on average, the insert, delete and search takes O ( log ⁡ n ) O(\log n) for n n nodes.
/// In the worst case, they degrade to that of a singly linked list: O ( n ) O(n). To address the boundless increase of the tree
/// height with arbitrary insertions and deletions, self-balancing variants of BSTs are introduced to bound the worst lookup complexity
/// to that of the binary logarithm. AVL trees were the first self-balancing binary search trees, invented in 1962 by Georgy Adelson-Velsky
/// and Evgenii Landis.
///
/// Binary search trees can be used to implement abstract data types such as dynamic sets, lookup tables and priority queues,
/// and used in sorting algorithms such as tree sort.
#[derive(Debug)]
pub struct BinarySearchTree<T: Copy + PartialOrd> {
    root: Option<Box<Node<T>>>,
}

impl<T: Copy + PartialOrd> Node<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
        }
    }
}

impl<T: Copy + PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    // Insert a value into the binary search tree
    pub fn insert(&mut self, elem: T) {
        let new_node = Box::new(Node::new(elem));

        let mut current = &mut self.root;

        // Use a loop to find the correct position for insertion
        while let Some(current_node) = current {
            if elem < current_node.elem {
                // Move to the left subtree
                current = &mut current_node.left;
            } else if elem > current_node.elem {
                // Move to the right subtree
                current = &mut current_node.right;
            } else {
                // If duplicate, ignore it
                return;
            }
        }

        // Insert the new node into the correct position
        *current = Some(new_node);
    }

    pub fn find(&self, elem: T) -> Option<T> {
        let mut current = self.root.as_ref();

        while let Some(current_node) = current {
            if current_node.elem == elem {
                return Some(elem);
            } else if elem < current_node.elem {
                current = current_node.left.as_ref();
            } else {
                current = current_node.right.as_ref();
            }
        }

        None
    }

    /// Traverse the BinarySearchTree breadth first
    ///
    /// It uses a queue to keep track of the nodes children order
    /// Time complexity is O(n) where n is the number of elements in the tree
    /// Space complexity is O(b) where b is the breadth of the tree
    pub fn breadth_first(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Push the tree root as the first node in the queue
        queue.push_back(self.root.as_ref());

        // Loop through the queue, and for each node:
        //   - add the node value to result
        //   - push the node children at the back of the queue
        while let Some(queue_head) = queue.pop_front() {
            match queue_head {
                None => continue,
                Some(node) => {
                    result.push(node.elem);
                    queue.push_back(node.left.as_ref());
                    queue.push_back(node.right.as_ref());
                }
            }
        }

        // Once the queue is empty, the whole tree as been traversed
        // return the result vector
        result
    }

    /// Traverse the BinarySearchTree depth first - pre order version
    ///
    /// It uses a queue to keep track of the nodes children order
    /// Time complexity is O(n) where n is the number of elements in the tree
    /// Space complexity is O(d) where d is the depth of the tree
    ///
    /// Note: The pre order version return an array for which the element as
    /// order as, if re-inserted into a binary search tree in the same order,
    /// the tree will be the same. It is a good candidate to serialize a binary tree
    /// into a flat structure
    pub fn depth_first_pre_order(&self) -> Vec<T> {
        let mut result = Vec::new();

        Self::traverse_depth_first_pre_order(self.root.as_ref(), &mut result);

        result
    }

    fn traverse_depth_first_pre_order(node: Option<&Box<Node<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            result.push(node.elem);
            Self::traverse_depth_first_pre_order(node.left.as_ref(), result);
            Self::traverse_depth_first_pre_order(node.right.as_ref(), result);
        }
    }

    /// Traverse the BinarySearchTree depth first - post order version
    ///
    /// It uses a queue to keep track of the nodes children order
    /// Time complexity is O(n) where n is the number of elements in the tree
    /// Space complexity is O(d) where d is the depth of the tree
    pub fn depth_first_post_order(&self) -> Vec<T> {
        let mut result = Vec::new();

        Self::traverse_depth_first_post_order(self.root.as_ref(), &mut result);

        result
    }

    fn traverse_depth_first_post_order(node: Option<&Box<Node<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::traverse_depth_first_post_order(node.left.as_ref(), result);
            Self::traverse_depth_first_post_order(node.right.as_ref(), result);
            result.push(node.elem);
        }
    }

    /// Traverse the BinarySearchTree depth first - in order version
    ///
    /// It uses a queue to keep track of the nodes children order
    /// Time complexity is O(n) where n is the number of elements in the tree
    /// Space complexity is O(d) where d is the depth of the tree
    ///
    /// Note: In order depth first traversal return the values of the tree sorted
    /// in croissant order.
    /// One usage could be to export the tree values sorted, in order to balance the tree
    pub fn depth_first_in_order(&self) -> Vec<T> {
        let mut result = Vec::new();

        Self::traverse_depth_first_in_order(self.root.as_ref(), &mut result);

        result
    }

    fn traverse_depth_first_in_order(node: Option<&Box<Node<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::traverse_depth_first_in_order(node.left.as_ref(), result);
            result.push(node.elem);
            Self::traverse_depth_first_in_order(node.right.as_ref(), result);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(7);
        tree.insert(14);
        tree.insert(6);
        tree.insert(8);
        tree.insert(13);
        tree.insert(15);
        // Duplicates should be ignored
        tree.insert(13);
        tree.insert(15);

        // println!("Tree: {:#?}", tree);

        assert_eq!(tree.root.as_ref().unwrap().elem, 10);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().elem, 7);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().elem, 14);
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .elem,
            6
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .elem,
            8
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .elem,
            13
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .elem,
            15
        );
    }

    #[test]
    fn test_find() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(30);

        // Should find present elements
        assert_eq!(tree.find(10), Some(10));
        assert_eq!(tree.find(5), Some(5));
        assert_eq!(tree.find(15), Some(15));
        assert_eq!(tree.find(12), Some(12));
        assert_eq!(tree.find(30), Some(30));

        // Should return None for absent elements
        assert_eq!(tree.find(1), None);
    }

    #[test]
    fn test_breadth_first_search() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(30);

        assert_eq!(tree.breadth_first(), vec![10, 5, 15, 12, 30]);
    }

    #[test]
    fn test_depth_first_pre_order() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(30);

        assert_eq!(tree.depth_first_pre_order(), vec![10, 5, 15, 12, 30]);
    }

    #[test]
    fn test_depth_first_post_order() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(30);

        assert_eq!(tree.depth_first_post_order(), vec![5, 12, 30, 15, 10]);
    }

    #[test]
    fn test_depth_first_in_order() {
        let mut tree = BinarySearchTree::new();

        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(30);

        assert_eq!(tree.depth_first_in_order(), vec![5, 10, 12, 15, 30]);
    }
}
