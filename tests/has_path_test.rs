use rust_dsa_with_integration_tests::graph::has_path::{has_path, rec};
use std::collections::HashMap;

#[test]
fn has_path_test() {

    let mut adj_list = HashMap::new();

    adj_list.insert(
        "Moscow".to_string(),
        vec![
            "Saint Petersburg".to_string(),
            "Novosibirsk".to_string(),
            "Barnaul".to_string(),
            "Omsk".to_string(),
        ],
    );
    adj_list.insert(
        "Saint Petersburg".to_string(),
        vec!["Moscow".to_string(), "Novosibirsk".to_string()],
    );
    adj_list.insert(
        "Novosibirsk".to_string(),
        vec!["Moscow".to_string(), "Saint Petersburg".to_string()],
    );
    adj_list.insert(
        "Omsk".to_string(),
        vec!["Moscow".to_string(), "Tyumen".to_string()],
    );
    adj_list.insert("Tyumen".to_string(), vec!["Novosibirsk".to_string()]);


    let result_1 = has_path(&adj_list, String::from("Moscow"), String::from("Barnaul"));

    let result_2 = has_path(&adj_list, String::from("Barnaul"), String::from("Moscow"));

    let result_3 = rec(&adj_list, String::from("Barnaul"), "Moscow".to_string());

    assert_eq!(true, result_1);
    assert_eq!(false, result_2);
    assert_eq!(false, result_3);
}