/// Complexity: O(n²)
pub fn bubble(ar: &mut [i32]) {
    let l = ar.len();

    let mut swapped = false;
    for i in 0..l {
        // the i th max element is already in the correct position
        // after iteration i, so we don't need to loop over that part
        for j in 1..(l - i) {
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
}

/// Complexity: O(n²)
pub fn bubble_recursive(ar: &mut [i32], n: usize) {
    if n == 1 {
        return;
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

#[cfg(test)]
mod tests {
    use super::{bubble, bubble_recursive};
    use crate::{rand_array, TEST_SIZE};

    #[test]
    fn test_regular() {
        let ar = &mut rand_array::<TEST_SIZE>();
        bubble(ar);
        assert!(ar.is_sorted())
    }
    #[test]
    fn test_recursive() {
        let ar = &mut rand_array::<TEST_SIZE>();
        bubble_recursive(ar, TEST_SIZE);
        assert!(ar.is_sorted())
    }
}
