/// Complexity: O(nlogn)
pub fn merge(ar: &mut [i32]) {
    mergesort(ar, 0, ar.len() - 1);
}

/// Both left and right are inclusive
fn mergesort(ar: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid = left + (right - left) / 2;

    mergesort(ar, left, mid);
    mergesort(ar, mid + 1, right);

    merge_ar(ar, left, mid, right);
}

fn merge_ar(ar: &mut [i32], left: usize, mid: usize, right: usize) {
    let left_len = mid - left + 1;
    let right_len = right - mid;

    let left_vec = Vec::from(&ar[left..=mid]);
    let right_vec = Vec::from(&ar[(mid + 1)..=right]);

    // current element in left vec
    let mut i = 0;
    // current element in right vec
    let mut j = 0;
    // current element in ar
    let mut k = left;

    while i < left_len && j < right_len {
        if left_vec[i] <= right_vec[j] {
            ar[k] = left_vec[i];
            i += 1;
        } else {
            ar[k] = right_vec[j];
            j += 1;
        }

        k += 1;
    }
    while i < left_len {
        ar[k] = left_vec[i];
        i += 1;
        k += 1;
    }
    while j < right_len {
        ar[k] = right_vec[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::merge;
    use crate::{rand_array, TEST_SIZE};

    #[test]
    fn test() {
        let ar = &mut rand_array::<TEST_SIZE>();
        merge(ar);
        assert!(ar.is_sorted());
    }
}
