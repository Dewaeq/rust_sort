pub fn quick(ar: &mut [i32]) -> &mut [i32] {
    quicksort(ar);
    ar
}

fn quicksort(ar: &mut [i32]) {
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

    quicksort(&mut ar[..left]);
    quicksort(&mut ar[right..]);
}

#[cfg(test)]
mod tests {
    use super::quicksort;
    use crate::rand_array;

    #[test]
    fn test() {
        let ar = &mut rand_array::<50>();
        quicksort(ar);
        assert!(ar.is_sorted());
    }
}
