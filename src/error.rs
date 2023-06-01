pub fn illegal_action(illegal_action: &str) {
    println!("{} is not a valid action. Consider using:", illegal_action);
    println!("-------");
    println!("Add: Add {{name}} to {{department}}");
    println!("Get: Get {{department}} (Optional)");
    println!("-------");
}

pub fn no_employees_in_department(department: &str) {
    println!("No employees in {}", department);
}
