use crate::LENGTH;

/// Complexity: O(n²)
pub fn bubble(mut ar: [i32; LENGTH], n: usize) -> [i32; LENGTH] {
    let mut swapped = false;
    for i in 0..n {
        // the i th max element is already in the correct position
        // after iteration i, so we don't need to loop over that part
        for j in 1..(n - i) {
            if ar[j - 1] > ar[j] {
                let temp = ar[j - 1];
                ar[j - 1] = ar[j];
                ar[j] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    ar
}

/// Complexity: O(n²)
pub fn bubble_recursive(mut ar: [i32; LENGTH], n: usize) -> [i32; LENGTH] {
    if n == 1 {
        return ar;
    }

    for i in 0..n {
        for j in 1..(n - i) {
            if ar[j - 1] > ar[j] {
                let temp = ar[j - 1];
                ar[j - 1] = ar[j];
                ar[j] = temp;
            }
        }
    }

    bubble_recursive(ar, n - 1)
}
