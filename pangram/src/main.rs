fn is_pangram(s: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let input_lower_case = s.to_lowercase();
    for n in alphabet.split("") {
        if !input_lower_case.contains(n) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
