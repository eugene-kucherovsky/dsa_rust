// Insertion Sort

// Time Complexity: O(n²) - where `n` is the number of elements
// Space complexity: O(1) - sorts elements in-place

pub fn insertion_sort_asc<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];

        while j > 0 && cur < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }

        arr[j] = cur;
    }
}

pub fn insertion_sort_desc<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];

        while j > 0 && arr[j - 1] < cur {
            arr.swap(j, j - 1);
            j = j - 1;
        }

        arr[j] = cur;
    }
}
