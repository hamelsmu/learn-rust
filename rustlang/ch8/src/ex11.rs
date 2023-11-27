// Using a hash map and vectors, create a text interface to allow 
// a user to add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department 
// or all people in the company by department, sorted alphabetically.

struct Employee {
    name: String,
    department: String,
}

struct Company {
    employees: Vec<Employee>,
}

impl Company {
    fn new() -> Company {
        Company { employees: Vec::new() }
    }

    fn add_employee(&mut self, name: String, department: String) {
        self.employees.push(Employee { name, department });
    }

    fn list_employees(&self, department: Option<&str>) {
        let mut employees = self.employees.clone();
        employees.sort_by(|a, b| a.name.cmp(&b.name)); // 

        match department {
            Some(department) => {
                for employee in employees {
                    if employee.department == department {
                        println!("{}", employee.name);
                    }
                }
            }
            None => {
                for employee in employees {
                    println!("{}: {}", employee.name, employee.department);
                }
            }
        }
    }
}

pub fn run(){
}