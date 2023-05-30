use rust_dsa_with_integration_tests::sorting::bubble_sort::{bubble_sort, bubble_sort_descending};

#[test]
fn bubble_sort_test() {
    let sorted = vec![-15, -12, 1, 2, 3, 4, 5, 6];
    let mut unsorted = vec![4, 2, 6, 1, 5, 3, -12, -15];
    bubble_sort(&mut unsorted);

    assert_eq!(sorted, unsorted);
}

#[test]
fn bubble_sort_descending_test() {
    let sorted_rev = vec![6, 5, 4, 3, 2, 1, -12, -15];

    let mut unsorted = vec![4, 2, 6, 1, 5, 3, -12, -15];
    bubble_sort_descending(&mut unsorted);

    assert_eq!(sorted_rev, unsorted);
}

#[test]
fn bubble_sort_if_sorted() {
    let sorted = vec![1, 2, 3, 4, 5, 6];
    let mut sorted_2 = vec![1, 2, 3, 4, 5, 6];
    bubble_sort(&mut sorted_2);

    assert_eq!(sorted, sorted_2);
}

#[test]
fn empty() {
    let mut empty_v: Vec<isize> = vec![];
    bubble_sort(&mut empty_v);
    assert_eq!(empty_v, empty_v);
}
