#[derive(Debug)]
pub enum ActionType {
    AddEmployee { name: String, department: String },
    GetDepartment { of_type: GetType },
    Invalid { action_name: String },
}
#[derive(Debug)]
pub enum GetType {
    All,
    Department { department_name: String },
}

use crate::modify;

pub fn command_to_action(command: &mut Vec<String>) -> ActionType {
    let action = command.remove(0).to_lowercase();
    match action.as_ref() {
        "add" => create_add_action(command),
        "get" => create_get_action(command),
        _ => ActionType::Invalid {
            action_name: action,
        },
    }
}

fn create_add_action(command: &mut Vec<String>) -> ActionType {
    let end_of_name = command.iter().position(|word| word == &"to");

    match end_of_name {
        Some(index) => {
            let name: Vec<String> = command.drain(..index).collect();
            let name = modify::convert_vec_to_string_with_spaces(&name);
            command.remove(0);
            let department = modify::convert_vec_to_string_with_spaces(&command);
            ActionType::AddEmployee { name, department }
        }
        None => ActionType::Invalid {
            action_name: "'blank'".to_string(),
        },
    }
}

fn create_get_action(command: &Vec<String>) -> ActionType {
    if command.len() == 0 || command.len() == 1 && command[0].to_lowercase() == "all" {
        ActionType::GetDepartment {
            of_type: GetType::All,
        }
    } else {
        let department = modify::convert_vec_to_string_with_spaces(&command);
        ActionType::GetDepartment {
            of_type: GetType::Department {
                department_name: department,
            },
        }
    }
}
