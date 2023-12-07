pub fn search<'a, T>(vec: &'a [T], val: &'a T) -> Option<&'a T>
where
    T: Eq,
{
    for item in vec {
        if item == val {
            return Some(item);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_should_work() {
        assert_eq!(search(&[1, 3, 2, 4], &4), Some(4).as_ref());
        assert_eq!(search(&[1, 3, 2, 4], &5), None);
    }
}
