use std::collections::HashMap;

use crate::{convert::GetType, error};

pub fn add_employee(
    name: String,
    department: String,
    company_registry: &mut HashMap<String, Vec<String>>,
) {
    println!("Adding {} to department '{}'", name, department);
    company_registry
        .entry(department)
        .or_insert(Vec::new())
        .push(name);
}

pub fn print_employees(get_type: GetType, registry: &HashMap<String, Vec<String>>) {
    match get_type {
        GetType::All => {
            for department in registry {
                print::department(&department.0, &mut department.1.to_owned())
            }
        }
        GetType::Department { department_name } => {
            let employees = registry.get(&department_name);
            match employees {
                Some(employees) => print::department(&department_name, &mut employees.to_owned()),
                None => error::no_employees_in_department(&department_name),
            }
        }
    }
}

pub mod print {

    pub fn department(department_name: &str, employees: &mut Vec<String>) {
        println!("Employees of Department: {}", department_name);
        employees.sort();
        for employee in employees {
            println!("- {}", employee)
        }
    }
}
