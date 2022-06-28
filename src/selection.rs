/// Complexity: O(n²)
pub fn selection(ar: &mut [i32]) {
    let l = ar.len();
    let mut min;

    // Loop to n - 1, so the max value of i is n - 2,
    // because if i were to be the last index (so n - 1),
    // the nested loop would be empty (n - 1 + 1)..n
    for i in 0..(l - 1) {
        min = i;

        for j in (i + 1)..l {
            // Check if there's a value past the current minimum's index
            // that's smaller than the current minimum value
            if ar[j] < ar[min] {
                let temp = ar[min];
                ar[min] = ar[j];
                ar[j] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::selection;
    use crate::{rand_array, TEST_SIZE};

    #[test]
    fn test() {
        let ar = &mut rand_array::<TEST_SIZE>();
        selection(ar);
        assert!(ar.is_sorted());
    }
}
