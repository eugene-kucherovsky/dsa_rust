// Merge Sort Algorithm
// Difficulty Level : Medium

// Time Complexity: O(n log n)
// Space Complexity: O(n)

pub fn merge_sort<T: PartialOrd + Copy>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut merged: Vec<_> = vec![];

    let mut i = 0;

    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

pub fn merge<T: PartialOrd + Copy>(vec: &Vec<T>) -> Vec<T> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge(&vec[0..size].to_vec());
        let right = merge(&vec[size..].to_vec());

        merge_sort(&left, &right) // return
    }
}
