#[derive(Debug)]
struct Employee {
    name: String,
    age: u128
}

struct Employees {
    list: Box<Vec<Employee>>
}

impl Employees {
    fn add_employee(&mut self, emp: Employee){
        self.list.push(emp)
    }

    fn remove_employee(&mut self, index: usize) {
        if index >= self.list.len() {
            panic!("index out of bounds")
        }
        self.list.remove(index);
    }

    fn print_employees(&self){
        self.list.iter().for_each(|emp| println!("{:?}", *emp));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Setup a helper function to create a new Employees instance
    fn setup_employees() -> Employees {
        Employees {
            list: Box::new(Vec::new()),
        }
    }

    #[test]
    fn test_add_employee() {
        let mut employees = setup_employees();
        employees.add_employee(Employee { name: String::from("Alice"), age: 30 });
        assert_eq!(employees.list.len(), 1);
        assert_eq!(employees.list[0].name, "Alice");
        assert_eq!(employees.list[0].age, 30);

        employees.add_employee(Employee { name: String::from("Bob"), age: 25 });
        assert_eq!(employees.list.len(), 2);
        assert_eq!(employees.list[1].name, "Bob");
        assert_eq!(employees.list[1].age, 25);
    }

    #[test]
    fn test_remove_employee() {
        let mut employees = setup_employees();
        employees.add_employee(Employee { name: String::from("Alice"), age: 30 });
        employees.add_employee(Employee { name: String::from("Bob"), age: 25 });
        
        employees.remove_employee(0);  // Remove Alice
        assert_eq!(employees.list.len(), 1);
        assert_eq!(employees.list[0].name, "Bob");

        employees.remove_employee(0);  // Remove Bob
        assert!(employees.list.is_empty());
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_remove_employee_out_of_bounds() {
        let mut employees = setup_employees();
        employees.add_employee(Employee { name: String::from("Alice"), age: 30 });
        employees.remove_employee(1);  // Try to remove non-existent index
    }
}
