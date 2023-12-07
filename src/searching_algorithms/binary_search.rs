pub fn search<'a, T>(vec: &'a [T], target: &'a T) -> Option<&'a T>
where
    T: Eq + Ord,
{
    let mut left = 0;
    let mut right = vec.len() - 1;

    while right >= left {
        let middle = (left + right) / 2;
        let middle_value = &vec[middle];

        match middle_value {
            m if m == target => return Some(target),
            m if m < target => left = middle + 1,
            m if m > target => right = middle - 1,
            _ => panic!("Something went wrong"),
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_should_work() {
        assert_eq!(search(&[1, 2, 3, 4], &4), Some(4).as_ref());
        assert_eq!(search(&[1, 2, 3, 4], &5), None);
    }
}
