/// Complexity: O(n²)
pub fn insertion<const L: usize>(mut ar: [i32; L]) -> [i32; L] {
    for i in 1..L {
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

    ar
}

/// Complexity: O(n²)]
pub fn insertion_recursive<const L: usize>(mut ar: [i32; L], n: usize) -> [i32; L] {
    if n <= 1 {
        return ar;
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
pub fn insertion_for<const L: usize>(mut ar: [i32; L]) -> [i32; L] {
    for i in 1..L {
        for j in 0..i {
            if ar[j] > ar[i] {
                ar[j..=i].rotate_right(1);
            }
        }
    }

    ar
}

#[cfg(test)]
mod tests {
    use super::{insertion, insertion_for, insertion_recursive};
    use crate::rand_array;

    #[test]
    fn test_for() {
        assert!(insertion_for(rand_array::<50>()).is_sorted())
    }

    #[test]
    fn test_regular() {
        assert!(insertion(rand_array::<50>()).is_sorted())
    }

    #[test]
    fn test_recursive() {
        assert!(insertion_recursive(rand_array::<50>(), 50).is_sorted())
    }
}
