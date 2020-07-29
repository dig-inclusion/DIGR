use std::string::String;

pub fn test_equals(actual: &String, expected: &String) -> bool {
    if actual == expected {
        return true;
    }
    return false;
}

pub fn test_greater_than(actual: &String, expected: &String) -> bool {
    if actual > expected {
        return true;
    }
    return false;
}

pub fn test_less_than(actual: &String, expected: &String) -> bool {
    if actual < expected {
        return true;
    }
    return false;
}

// pub fn assert_or_assert_equals(current_value: &String, expected_value: &String, elem: &String) ->  Option<()> {

//     assert!(current_value == CURR_VARIABLE, "Unexpected test case element {}", current_value);

//     if elem > expected_value {
//         return Some(());
//     }
//     return None;
// }
