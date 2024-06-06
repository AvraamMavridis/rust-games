## Exercise: Manage a Dynamic List of Employee Ages Using `Box`

### Background
In a small software startup, you need to keep track of various details about employees, such as their name and age. The number of employees can change dynamically as new hires are made and others leave. Managing this data efficiently requires using heap allocation.

### Requirements
1. Define an `Employee` struct with at least two fields: name: String and age: u32.
2. Use Box to allocate the vector of employees on the heap.
3. Use an `Employees` struct to hold the vector
4. Implement the following functions for the `Employees`:
   - `add_employee`: Takes a mutable reference to a boxed vector of employees and an employee, adding the employee to the vector.
   - `remove_employee`: Takes a mutable reference to a boxed vector of employees and an index, removes the employee at that index. Ensure out-of-bounds safety.
   - `print_employees`: Accepts a boxed vector of employees and prints all employee details.

