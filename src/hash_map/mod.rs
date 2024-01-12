// Those are just constants for the hash function
// (Hash function is better with those constant set values that are prime number)
const MAP_SIZE: usize = 3; // This is set very low for testing purpose
const HASH_CONSTANT: usize = 31;

/// A simple hash map implementation for pedagogical purpose. It uses separate chaining to handle collisions
///
/// A hashtable, often referred to as a hash map, is a data structure that enables efficient
/// storage and retrieval of key-value pairs. It employs a hashing function to map keys to indices
/// within an array, where the associated values are stored. The use of a hash function allows
/// for rapid lookup and insertion of values based on their corresponding keys, making
/// hashtables well-suited for scenarios requiring fast access to data. Collisions,
/// where two different keys hash to the same index, are typically handled by employing
/// techniques such as chaining (using linked lists or other structures) or open addressing.
/// Hashtables provide constant-time average-case complexity for basic operations, making them a
/// fundamental component in various applications, including databases, caches, and language implementations.
///
/// ## Handling collisions
///
/// Collisions in a hashmap occur when two different keys hash to the same index. There are several
/// techniques to handle collisions, and two common approaches are chaining and open addressing.
///
/// ### Chaining:
///
/// In chaining, each slot in the hashtable contains a linked list (or another data structure)
/// of key-value pairs that hash to the same index. When a collision occurs, the new key-value
/// pair is simply appended to the linked list at that index. Retrieving a value involves
/// traversing the linked list associated with the hash index.
///
/// Chaining is straightforward and easy to implement. It effectively handles a large number
/// of collisions, and its performance is generally good. However, it may lead to
/// increased memory usage due to the storage of additional pointers for each linked list node.
///
/// ### Open addressing:
///
/// In open addressing, all elements are stored directly in the array. When a collision occurs,
/// the algorithm searches for the next available slot in the array. There are different probing
/// techniques, such as linear probing, quadratic probing, and double hashing, which determine
/// the sequence of slots to be examined.
///
/// Linear probing involves checking the next slot in sequence until an empty slot is found.
/// Quadratic probing uses a quadratic function to determine the next slot to check.
/// Double hashing involves using a second hash function to calculate the step size for probing.
///
/// Open addressing can be more memory-efficient than chaining, as it avoids the need for
/// additional data structures to store collisions. However, it may experience clustering,
/// where consecutive slots become occupied, potentially impacting performance.
#[derive(Debug)]
pub struct HashMap<V: Clone> {
    /// key-value pairs are store in `key_map`. It's a 2 dimensional vectors.
    /// The first dimension is indexed by the hash of the inserted keys.
    /// In case of collisions, several key-value pairs can be stored at the same hash
    pub key_map: Vec<Vec<(String, V)>>,
}

impl<V: Clone> HashMap<V> {
    /// Create an empty hashmap
    pub fn new() -> Self {
        Self {
            key_map: vec![Vec::new(); MAP_SIZE as usize],
        }
    }

    /// Insert a key value pair, or update it if it already exists
    pub fn set(&mut self, key: String, value: V) {
        // Get the hash of the key
        let index = Self::hash(&key);

        // Try to find the item
        let item = self.key_map[index].iter_mut().find(|item| item.0 == key);

        // If it does no exists, add it
        // or update its value
        match item {
            None => self.key_map[index].push((key, value)),
            Some(item) => item.1 = value,
        }
    }

    /// Given a key, retrieve the associated value
    pub fn get(&self, key: String) -> Option<&V> {
        // Get the hash of the key
        let index = Self::hash(&key);

        // Retrieve the associated value
        self.key_map
            .get(index)
            .and_then(|bucket| bucket.into_iter().find(|item| item.0 == key))
            .map(|item| &item.1)
    }

    /// Given a key, delete the associated value and return it
    pub fn delete(&mut self, key: String) -> Option<V> {
        // Get the hash of the key
        let index = Self::hash(&key);

        // Remove the associated value from te hash map and return it
        self.key_map[index]
            .iter()
            .position(|item| item.0 == key)
            .map(|item_position| self.key_map[index].remove(item_position).1)
    }

    fn hash(key: &String) -> usize {
        key.chars().take(100).fold(0, |acc, char| {
            (acc * HASH_CONSTANT + char as usize) % MAP_SIZE
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_map_without_collisions() {
        let mut hash_map: HashMap<u32> = HashMap::new();

        // If no value, get should return None
        assert_eq!(hash_map.get("Apple".to_string()), None);

        // If there is a value, get should return it
        hash_map.set("Apples".to_string(), 3);
        assert_eq!(hash_map.get("Apples".to_string()), Some(&3));

        // If setting on an existing value, it should update it
        hash_map.set("Apples".to_string(), 10);
        assert_eq!(hash_map.get("Apples".to_string()), Some(&10));

        // Delete an existing value should remove it and return it
        assert_eq!(hash_map.delete("Apples".to_string()), Some(10));
        assert_eq!(hash_map.get("Apples".to_string()), None);
    }

    #[test]
    fn test_hash_map_with_collisions() {
        let mut hash_map: HashMap<u32> = HashMap::new();

        // Everything should work if there is several values in
        // one bucket (when there is collisions)
        assert_eq!(HashMap::<u32>::hash(&"Apples".to_string()), 1);
        assert_eq!(HashMap::<u32>::hash(&"Melons".to_string()), 1);
        hash_map.set("Melons".to_string(), 3);

        // If no value, get should return None
        assert_eq!(hash_map.get("Apple".to_string()), None);

        // If there is a value, get should return it
        hash_map.set("Apples".to_string(), 3);
        assert_eq!(hash_map.get("Apples".to_string()), Some(&3));

        // If setting on an existing value, it should update it
        hash_map.set("Apples".to_string(), 10);
        assert_eq!(hash_map.get("Apples".to_string()), Some(&10));

        // Delete an existing value should remove it and return it
        assert_eq!(hash_map.delete("Apples".to_string()), Some(10));
        assert_eq!(hash_map.get("Apples".to_string()), None);
    }
}
