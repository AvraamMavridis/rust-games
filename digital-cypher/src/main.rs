use std::hash::hasMa

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fixed_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }
}
