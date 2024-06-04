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
}
