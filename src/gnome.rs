use crate::LENGTH;

/// Complexity: O(n²)
pub fn gnome(mut ar: [i32; LENGTH], n: usize) -> [i32; LENGTH] {
    let mut i = 0;

    while i < n {
        if i == 0 || ar[i] > ar[i - 1] {
            i += 1;
        } else {
            let temp = ar[i - 1];
            ar[i - 1] = ar[i];
            ar[i] = temp;
            i -= 1;
        }
    }

    ar
}
