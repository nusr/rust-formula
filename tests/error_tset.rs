// tests/error_test.rs
use formula::error::ErrorTypes;

#[test]
fn test_div0_to_string() {
    let error = ErrorTypes::Div0;
    assert_eq!(error.to_string(), "#DIV/0!");
}

#[test]
fn test_name_to_string() {
    let error = ErrorTypes::Name;
    assert_eq!(error.to_string(), "#NAME?");
}

#[test]
fn test_na_to_string() {
    let error = ErrorTypes::NA;
    assert_eq!(error.to_string(), "#N/A");
}

#[test]
fn test_null_to_string() {
    let error = ErrorTypes::Null;
    assert_eq!(error.to_string(), "#NULL!");
}

#[test]
fn test_num_to_string() {
    let error = ErrorTypes::Num;
    assert_eq!(error.to_string(), "#NUM!");
}

#[test]
fn test_ref_to_string() {
    let error = ErrorTypes::Ref;
    assert_eq!(error.to_string(), "#REF!");
}

#[test]
fn test_value_to_string() {
    let error = ErrorTypes::Value;
    assert_eq!(error.to_string(), "#VALUE!");
}

#[test]
fn test_getting_data_to_string() {
    let error = ErrorTypes::GettingData;
    assert_eq!(error.to_string(), "#GETTING_DATA");
}