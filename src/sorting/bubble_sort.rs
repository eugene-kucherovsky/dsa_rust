// Bubble Sort

// Time Complexity: O(n²)
// Auxiliary Space: O(1)

pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    for i in 0..arr.len() {
        let mut sorted = true;

        for j in 0..(arr.len() - 1) - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}

pub fn bubble_sort_descending<T: PartialOrd>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    for i in 0..arr.len() {
        let mut sorted = true;

        for j in 0..(arr.len() - 1) - i {
            // just change the condition
            if arr[j] < arr[j + 1] {
                arr.swap(j, j + 1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}
