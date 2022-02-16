use crate::LENGTH;

/// Complexity: O(n²)
pub fn insertion(mut ar: [i32; LENGTH], n: usize) -> [i32; LENGTH] {
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

    ar
}

/// Complexity: O(n²)
pub fn insertion_recursive(mut ar: [i32; LENGTH], n: usize) -> [i32; LENGTH] {
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
pub fn insertion_for(ar: [i32; LENGTH], n: usize) -> Vec<i32> {
    let mut ar = Vec::from(ar);
    for i in 1..n {
        for j in 0..i {
            if ar[j] > ar[i] {
                let temp = ar[i];
                ar.remove(i);
                ar.insert(j, temp);
                break;
            }
        }
    }

    ar
}
