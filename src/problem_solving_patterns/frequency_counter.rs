use std::collections::HashMap;
use std::hash::Hash;

/// Generic group by
///
/// Generic group by function. It uses a hash map as a frequency aggregator
pub fn group_by<T, K, F>(data: Vec<T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut result: HashMap<K, Vec<T>> = HashMap::new();

    for item in data {
        let key = key_fn(&item);
        result.entry(key).or_insert(Vec::new()).push(item);
    }

    result
}

/// Given two positive integers, find out if the two numbers have the same frequency of digits.
///
/// Solution have the following complexities:
/// Time: O(N)
pub fn same_frequency(a: i32, b: i32) -> bool {
    let mut counter = a.to_string().chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    for c in b.to_string().chars() {
        let default_value = &mut 0;
        let c_count = counter.get_mut(&c).unwrap_or(default_value);

        if c_count == &mut 0 {
            return false;
        } else {
            *c_count -= 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn group_by_should_work() {
        #[derive(Debug)]
        struct Item {
            item_type: String,
        }

        let items = vec![
            Item {
                item_type: "Type 1".to_string(),
            },
            Item {
                item_type: "Type 2".to_string(),
            },
            Item {
                item_type: "Type 3".to_string(),
            },
            Item {
                item_type: "Type 1".to_string(),
            },
        ];

        let result = group_by(items, |item| item.item_type.clone());

        println!("{:?}", result);

        assert!(
            result.get("Type 1").unwrap().len() == 2
                && result.get("Type 2").unwrap().len() == 1
                && result.get("Type 3").unwrap().len() == 1
        );
    }

    #[test]
    fn same_frequency_should_work() {
        assert_eq!(same_frequency(182, 281), true);
        assert_eq!(same_frequency(3589578, 5879385), true);
        assert_eq!(same_frequency(34, 14), false);
        assert_eq!(same_frequency(22, 222), false);
    }
}
