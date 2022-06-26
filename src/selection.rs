/// Complexity: O(n²)
pub fn selection<const L: usize>(mut ar: [i32; L]) -> [i32; L] {
    let mut min;

    // Loop to n - 1, so the max value of i is n - 2,
    // because if i were to be the last index (so n - 1),
    // the nested loop would be empty (n - 1 + 1)..n
    for i in 0..(L - 1) {
        min = i;

        for j in (i + 1)..L {
            // Check if there's a value past the current minimum's index
            // that's smaller than the current minimum value
            if ar[j] < ar[min] {
                let temp = ar[min];
                ar[min] = ar[j];
                ar[j] = temp;
            }
        }
    }

    ar
}

#[cfg(test)]
mod tests {
    use crate::rand_array;

    use super::selection;

    #[test]
    fn test() {
        assert!(selection(rand_array::<50>()).is_sorted())
    }
}
