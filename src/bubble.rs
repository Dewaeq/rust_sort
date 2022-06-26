/// Complexity: O(n²)
pub fn bubble<const L: usize>(mut ar: [i32; L]) -> [i32; L] {
    let mut swapped = false;
    for i in 0..L {
        // the i th max element is already in the correct position
        // after iteration i, so we don't need to loop over that part
        for j in 1..(L - i) {
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
pub fn bubble_recursive<const L: usize>(mut ar: [i32; L], n: usize) -> [i32; L] {
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

#[cfg(test)]
mod tests {
    use crate::rand_array;

    use super::{bubble, bubble_recursive};

    #[test]
    fn test_regular() {
        assert!(bubble(rand_array::<50>()).is_sorted())
    }
    #[test]
    fn test_recursive() {
        assert!(bubble_recursive(rand_array::<50>(), 50).is_sorted())
    }
}
