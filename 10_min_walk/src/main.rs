use std::collections::HashMap;

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }

    let mut steps = HashMap::new();

    for c in walk {
        *steps.entry(*c).or_insert(0) += 1;
    }

    steps.get(&'n') == steps.get(&'s') && steps.get(&'w') == steps.get(&'e')
}

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
