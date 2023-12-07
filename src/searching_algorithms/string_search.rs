pub fn count_str_match(str: &str, sub_str: &str) -> i32 {
    if sub_str > str {
        return 0;
    }

    let mut counter = 0;

    'main: for i in 0..=(str.chars().count() - sub_str.chars().count()) {
        'compare: for (j, c) in sub_str.chars().enumerate() {
            if str.chars().nth(i + j).unwrap() == c {
                continue 'compare;
            } else {
                continue 'main;
            }
        }

        counter += 1;
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_str_match_should_work() {
        assert_eq!(count_str_match("no", "noo"), 0);
        assert_eq!(count_str_match("wawowu", "no"), 0);
        assert_eq!(count_str_match("wawowu", "wa"), 1);
        assert_eq!(count_str_match("wawawa", "wa"), 3);
    }
}
