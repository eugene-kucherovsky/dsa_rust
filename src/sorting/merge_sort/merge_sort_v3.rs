// Merge Sort Algorithm
// Difficulty Level : Medium

// Time Complexity: O(n log n)
// Space Complexity: O(n)

pub fn merge<T: Ord + Copy>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }

    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }

    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }

    let mid = arr.len() / 2;

    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    let mut y: Vec<T> = arr.to_vec();

    merge(&arr[0..mid], &arr[mid..], &mut y[..]);

    arr.copy_from_slice(&y);
}
