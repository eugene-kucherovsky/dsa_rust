use rust_dsa_with_integration_tests::sorting::merge_sort::merge_sort_v3::merge_sort;

#[test]
fn merge_sort_v3_test() {
    let sorted_numbers = vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let mut unsorted_numbers = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    merge_sort(&mut unsorted_numbers);
    assert_eq!(sorted_numbers, unsorted_numbers);

    let sorted_strings = vec!["airplane", "art", "beach", "car", "hotel", "house"];
    let mut unsorted_strings = vec!["beach", "hotel", "airplane", "car", "house", "art"];
    merge_sort(&mut unsorted_strings);
    assert_eq!(sorted_strings, unsorted_strings);
}
