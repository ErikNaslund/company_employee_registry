pub fn convert_vec_to_string_with_spaces(name: &Vec<String>) -> String {
    let mut full_name = String::new();
    for name_part in name {
        full_name += name_part;
        full_name.push(' ');
    }
    full_name.trim().to_owned()
}
