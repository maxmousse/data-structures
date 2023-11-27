pub fn reverse(str: &str) -> String {
    if str.is_empty() {
        return "".to_string();
    }

    let char = str.chars().last().unwrap();

    char.to_string() + &reverse(&str[..str.len() - 1])
}

pub fn reverse_tail<'a>(str: &'a str, tail: &'a mut String) -> &'a mut String {
    if str.is_empty() {
        return tail;
    }
    tail.push(str.chars().last().unwrap());
    reverse_tail(&str[..str.len() - 1], tail)
}

pub fn is_palindrome(str: &str) -> bool {
    if str.len() <= 1 {
        return true;
    }

    if str.chars().next().unwrap() == str.chars().last().unwrap() {
        return is_palindrome(&str[1..str.len() - 1]);
    } else {
        false
    }
}

pub fn some<'a, T, P>(vec: &'a [T], predicate: P) -> bool
where
    P: Fn(&'a T) -> bool,
{
    if vec.len() == 0 {
        return false;
    } else if predicate(vec.first().unwrap()) {
        return true;
    } else {
        some(&vec[1..], predicate)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_should_work() {
        assert_eq!(reverse("awesome"), "emosewa");
        assert_eq!(reverse("rithmschool"), "loohcsmhtir");
    }

    #[test]
    fn reverse_tail_should_work() {
        // Test case 1: Non-empty string
        let input_str = "Hello, World!";
        let mut reversed_str = String::new();
        reverse_tail(input_str, &mut reversed_str);
        assert_eq!(reversed_str, "!dlroW ,olleH");

        // Test case 2: Empty string
        let input_str_empty = "";
        let mut reversed_str_empty = String::new();
        reverse_tail(input_str_empty, &mut reversed_str_empty);
        assert_eq!(reversed_str_empty, "");

        // Test case 3: String with a single character
        let input_str_single = "A";
        let mut reversed_str_single = String::new();
        reverse_tail(input_str_single, &mut reversed_str_single);
        assert_eq!(reversed_str_single, "A");
    }

    #[test]
    fn is_palindrome_should_work() {
        assert_eq!(is_palindrome("awesome"), false);
        assert_eq!(is_palindrome("foobar"), false);
        assert_eq!(is_palindrome("tacocat"), true);
        assert_eq!(is_palindrome("amanaplanacanalpanama"), true);
        assert_eq!(is_palindrome("amanaplanacanalpandemonium"), false);
    }

    #[test]
    fn some_should_work() {
        let is_odd = |t| (t % 2 == 0);
        assert_eq!(some(&[1, 3, 4], is_odd), true);
        assert_eq!(some(&[1, 3, 5], is_odd), false);
    }
}
