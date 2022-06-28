/// Complexity: O(nlog(n))
pub fn quick(ar: &mut [i32]) {
    match ar.len() {
        0 | 1 => return,
        2 => {
            if ar[0] > ar[1] {
                ar.swap(0, 1);
            }
            return;
        }
        _ => (),
    }

    let (pivot, slice) = ar.split_last_mut().unwrap();
    // Everything before left is <= pivot
    let mut left: usize = 0;
    // Everything after right is > pivot
    let mut right: usize = slice.len() - 1;

    while left <= right {
        if &slice[left] <= pivot {
            left += 1;
        } else if &slice[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            slice.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // Move the pivot to the correct position
    ar.swap(ar.len() - 1, left);

    quick(&mut ar[..left]);
    quick(&mut ar[right..]);
}

#[cfg(test)]
mod tests {
    use super::quick;
    use crate::{rand_array, TEST_SIZE};

    #[test]
    fn test() {
        let ar = &mut rand_array::<TEST_SIZE>();
        quick(ar);
        assert!(ar.is_sorted());
    }
}
