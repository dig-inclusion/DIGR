use std::string::String;

const CURR_VARIABLE:&str = "$element";

    // fn if_or_if_equals(current_value: &str, expected_value: String, elem: String) -> &str;
//     fn if_not_equals(str, String) -> str;

//     fn if_greater_than(str, String) -> str;
//     fn if_less_than(str, String) -> str;
//     fn if_greater_than_or_equals(str, String) -> str;
//     fn if_less_than_or_equals(str, String) -> str;
//     fn if_null(str, String) -> str;
//     fn if_not_null(str, String) -> str;
//     fn if_includes(str, String) -> str;
//  
//     fn assert_or_assert_equals(str, String) -> str;
//     fn assert_greater_than(str, String) -> str;
//     fn assert_less_than(str, String) -> str;
//     fn assert_greater_than_or_equals(str, String) -> str;
//     fn assert_less_than_or_equals(str, String) -> str;
//     fn assert_null(str, String) -> str;
//  
//     fn assert_not_greater_than(str, String) -> str;
//     fn assert_not_equals(str, String) -> str;
//     fn assert_not_null(str, String) -> str;


pub fn if_or_if_equals(current_value: &String, expected_value: &String, elem: &String) -> Option<()> {

    assert!(current_value == CURR_VARIABLE, "Unexpected test case element {}", current_value);
    
    if elem == expected_value {
        return Some(());
    }
    return None;
}


pub fn if_greater_than(current_value: &String, expected_value: &String, elem: &String) ->  Option<()> {

    assert!(current_value == CURR_VARIABLE, "Unexpected test case element {}", current_value);
    
    if elem > expected_value {
        return Some(());
    }
    return None;
}


