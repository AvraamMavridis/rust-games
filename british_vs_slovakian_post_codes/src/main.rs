extern crate regex;

use regex::Regex;

fn which_postcode(s: &str) -> String {
    let parts: Vec<&str> = s.trim().split(" ").collect();

    if parts.len() != 2 || parts[0].contains(" ") || parts[1].contains(" "){
        return String::from("Not valid");
    }

    if matches!((parts[0].len(), parts[1].len()), (3, 2))
        && parts[0].parse::<u32>().is_ok()
        && parts[1].parse::<u32>().is_ok()
    {
        return String::from("SK");
    }

    let gb_regex = Regex::new(r"^[A-Za-z]{1,2}\d{1,2} \d[A-Za-z]{2}$").unwrap();

    if gb_regex.is_match(s.trim()) {
        return String::from("GB");
    }

    String::from("Not valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(s: &str, exp: &str) {
        let exp = exp.to_string();
        let actual = which_postcode(s);
        assert_eq!(actual, exp, "\"{}\" should return \"{}\"", s, exp);
    }

    #[test]
    fn test_gb() {
        do_test("G4 7AH", "GB");
        do_test("G12 8NU", "GB");
        do_test("dN1 1AA", "GB");
        do_test("Se21 7AA", "GB");
        do_test("G4 7Ah  ", "GB");
    }
    #[test]
    fn test_sk() {
        do_test("040 01", "SK");
        do_test("070 08", "SK");
        do_test("  810 08", "SK");
    }

    #[test]
    fn test_not_valid() {
        do_test("G4  7AH", "Not valid");
        do_test("12 8NU", "Not valid");
        do_test("DN1 AAA", "Not valid");
        do_test("SE21 AA7", "Not valid");
        do_test("G47AH", "Not valid");
        do_test("04001", "Not valid");
        do_test("810  08", "Not valid")
    }
}
