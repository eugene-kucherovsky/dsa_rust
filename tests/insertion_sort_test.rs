use rust_dsa_with_integration_tests::sorting::insertion_sort::{
    insertion_sort_asc, insertion_sort_desc,
};

// cargo test insertion_sort_asc_test
#[test]
fn insertion_sort_asc_test() {
    let sorted = vec![-15, -12, 1, 2, 3, 4, 5, 6];

    let mut unsorted = vec![4, 2, 6, 1, 5, 3, -12, -15];

    insertion_sort_asc(&mut unsorted);

    assert_eq!(sorted, unsorted);
}
// cargo test insertion_sort_desc_test
#[test]
fn insertion_sort_desc_test() {
    let sorted_rev = vec![6, 5, 4, 3, 2, 1, -12, -15];

    let mut unsorted = vec![4, 2, 6, 1, 5, 3, -12, -15];

    insertion_sort_desc(&mut unsorted);

    assert_eq!(sorted_rev, unsorted);
}
