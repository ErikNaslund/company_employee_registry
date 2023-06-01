use std::{collections::HashMap, io};

mod action;
mod convert;
mod error;
mod modify;
fn main() {
    let mut company_registry: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = prompt_user_input();
        let action = convert::command_to_action(&mut command);
        match action {
            convert::ActionType::AddEmployee { name, department } => {
                action::add_employee(name, department, &mut company_registry)
            }
            convert::ActionType::GetDepartment { with_name: of_type } => {
                action::print_employees(of_type, &company_registry)
            }
            convert::ActionType::Invalid { action_name } => error::illegal_action(&action_name),
        }
    }
}

fn prompt_user_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}
