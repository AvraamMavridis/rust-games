use std::vec::IntoIter;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct CustomCollection {
    _collection: Vec<i32>
}

impl CustomCollection {
    fn new(col: Vec<i32>) -> CustomCollection {
        CustomCollection {
            _collection: col
        }
    }
}

trait Sum {
    fn sum(&self) -> i32;
}

trait Product {
    fn product(&self) -> i32;
}

impl Sum for CustomCollection {
    fn sum(&self) -> i32 {
        self._collection.iter().sum::<i32>()
    }
}

impl Product for CustomCollection {
    fn product(&self) -> i32 {
        self._collection.iter().product()
    }
}

struct CustomCollectionIter {
    iter: IntoIter<i32>,
}

impl Iterator for CustomCollectionIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl IntoIterator for CustomCollection {
    type Item = i32;
    type IntoIter = CustomCollectionIter;

    fn into_iter(self) -> Self::IntoIter {
        CustomCollectionIter {
            iter: self._collection.into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let collection = CustomCollection::new(vec![1, 2, 3, 4, 5]);
        let mut iter = collection.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_debug() {
        let collection = CustomCollection::new(vec![1, 2, 3, 4, 5]);
        let debug_output = format!("{:?}", collection);
        assert_eq!(debug_output, "CustomCollection { _collection: [1, 2, 3, 4, 5] }");
    }

    #[test]
    fn test_into_iterator() {
        let collection = CustomCollection::new(vec![1, 2, 3, 4, 5]);
        let mut iter = collection.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_sum() {
        let collection = CustomCollection::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(collection.sum(), 15);
    }

    #[test]
    fn test_product() {
        let collection = CustomCollection::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(collection.product(), 120);
    }
}
