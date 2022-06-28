/// Complexity: O(n²)
pub fn insertion(ar: &mut [i32]) {
    for i in 1..ar.len() {
        let key = ar[i];
        let mut j = i - 1;

        while key < ar[j] {
            ar[j + 1] = ar[j];
            ar[j] = key;

            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
    }
}

/// Complexity: O(n²)]
pub fn insertion_recursive(ar: &mut [i32], n: usize) {
    if n <= 1 {
        return;
    }

    for i in 1..n {
        let key = ar[i];
        let mut j = i - 1;

        while key < ar[j] {
            ar[j + 1] = ar[j];
            ar[j] = key;

            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
    }

    insertion_recursive(ar, n - 1)
}

/// Same as normal insertion sort, but with a for loop
/// Complexity: O(n²)
pub fn insertion_for(ar: &mut [i32]) {
    for i in 1..ar.len() {
        for j in 0..i {
            if ar[j] > ar[i] {
                ar[j..=i].rotate_right(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{insertion, insertion_for, insertion_recursive};
    use crate::{rand_array, TEST_SIZE};

    #[test]
    fn test_for() {
        let ar = &mut rand_array::<TEST_SIZE>();
        insertion_for(ar);
        assert!(ar.is_sorted())
    }

    #[test]
    fn test_regular() {
        let ar = &mut rand_array::<TEST_SIZE>();
        insertion(ar);
        assert!(ar.is_sorted())
    }

    #[test]
    fn test_recursive() {
        let ar = &mut rand_array::<TEST_SIZE>();
        insertion_recursive(ar, TEST_SIZE);
        assert!(ar.is_sorted())
    }
}
