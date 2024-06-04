fn count_chars(s: String) -> String {
    let mut prev: Option<char> = None;
    let mut result = String::new();
    let mut counter: u32 = 0;

    for f in s.chars() {
        if Some(f) != prev {
            if let Some(p) = prev {
                result.push(p);
                result.push_str(&counter.to_string());
            }
            counter = 1;
        } else {
            counter += 1;
        }
        prev = Some(f);
    }

    if let Some(p) = prev {
        result.push(p);
        result.push_str(&counter.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_test() {
        let exp = "a3b5c4d1a2";
        let actual = count_chars(String::from("aaabbbbbccccdaa"));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_empty_string() {
        let exp = "";
        let actual = count_chars(String::from(""));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_single_character() {
        let exp = "a1";
        let actual = count_chars(String::from("a"));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_repeated_single_character() {
        let exp = "a5";
        let actual = count_chars(String::from("aaaaa"));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_multiple_characters() {
        let exp = "a1b1c1";
        let actual = count_chars(String::from("abc"));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_mixed_characters() {
        let exp = "a2b1c3a1";
        let actual = count_chars(String::from("aabccca"));
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_numbers_in_string() {
        let exp = "a2b2c2d2";
        let actual = count_chars(String::from("aabbccdd"));
        assert_eq!(actual, exp);
    }
}
