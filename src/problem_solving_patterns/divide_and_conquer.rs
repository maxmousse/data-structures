/// Binary search in an ordered vector
///
/// Given an ordered vector, uses dichotomy to quickly find
/// the searched value
pub fn b_search(vec: &Vec<i32>, target: i32) -> Option<usize> {
    // Quick fail if the source vector is empty
    if vec.len() == 0 {
        return None;
    }

    // Init pointers to the vector extremities
    let mut left_pointer = 0;
    let mut right_pointer = vec.len() - 1;

    while left_pointer <= right_pointer {
        // Find the middle of the current window
        let middle_pointer = (left_pointer + right_pointer) / 2;

        // Compare its value with the target value
        let middle_pointer_val = vec.get(middle_pointer).unwrap();

        // If we found the value, return
        // else move to the relevant half window
        match middle_pointer_val {
            v if *v == target => return Some(middle_pointer),
            v if *v < target => left_pointer = middle_pointer + 1,
            v if *v > target => right_pointer = middle_pointer - 1,
            _ => return None,
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn b_search_should_work() {
        assert!(b_search(&vec![], 10) == None);
        assert!(b_search(&vec![1, 2, 10, 20, 30], 10) == Some(2));
        assert!(b_search(&vec![1, 2, 20, 30], 10) == None);
    }
}
