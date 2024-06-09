# Rust Traits Challenge: Implement a Custom Iterator

## Objective:
Create a custom iterator for a collection and implement various traits to provide additional functionality.

## Requirements:

### Define a Custom Collection:
- Create a struct `CustomCollection` that holds a `Vec<i32>`.

### Implement the Iterator Trait:
- Implement the `Iterator` trait for `CustomCollection`.
- Define the associated `Item` type as `i32`.
- Implement the `next` method.

### Additional Traits:
- Implement the `Debug` trait for `CustomCollection` to print the contents of the collection.
- Implement the `IntoIterator` trait for `CustomCollection` to allow easy iteration.

### Extended Functionality:
- Implement a trait `Sum` that provides a method `sum(&self) -> i32` to sum all elements in the collection.
- Implement a trait `Product` that provides a method `product(&self) -> i32` to compute the product of all elements in the collection.
