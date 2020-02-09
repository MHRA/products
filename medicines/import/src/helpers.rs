pub fn is_string_in_list(string_list: &Vec<String>, string_to_check: &String) -> bool {
  if string_list.iter().any(|i| i == string_to_check) {
    return true;
  }
  return false;
}
