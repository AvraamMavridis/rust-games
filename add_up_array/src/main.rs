use std::collections::HashSet;

fn add_up(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }

    let mut seen: HashSet<i32> = HashSet::new();

    for i in nums.iter() {
        if seen.contains(&(k - i)){
            return true;
        }
        seen.insert(*i);
    }

    return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_up() {
        assert_eq!(add_up(vec![10, 15, 3, 7], 17), true);
        assert_eq!(add_up(vec![1, 2, 3, 4, 5], 9), true);
        assert_eq!(add_up(vec![1, 2, 3, 4, 5], 10), false);
        assert_eq!(add_up(vec![], 0), false);
        assert_eq!(add_up(vec![5, 5, 5, 5], 10), true);
        assert_eq!(add_up(vec![1, 2, 3, 4, 5], 8), true);
        assert_eq!(add_up(vec![1, 2, 3, 4, 5], 2), false);
    }
}
