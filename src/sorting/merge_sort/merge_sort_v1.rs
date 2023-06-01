// Merge Sort Algorithm
// Difficulty Level : Medium

// Time Complexity: O(n log n)
// Space Complexity: O(n)

pub fn merge_sort<T: PartialOrd + Clone>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
    let mut result = vec![];

    while left.len() > 0 && right.len() > 0 {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    while left.len() > 0 {
        result.push(left.remove(0));
    }

    while right.len() > 0 {
        result.push(right.remove(0));
    }

    return result;
}

pub fn merge<T: PartialOrd + Clone>(array: Vec<T>) -> Vec<T> {
    if array.len() < 2 {
        return array;
    };

    let mid = array.len() / 2;
    let left: Vec<_> = array[..mid].to_vec();
    let right: Vec<_> = array[mid..].to_vec();

    merge_sort(merge(left), merge(right))
}
