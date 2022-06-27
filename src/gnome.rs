/// Wonderful explanation by Dick Grune:
///
/// Gnome Sort is based on the technique used by the standard Dutch Garden Gnome (Du.: tuinkabouter).
///
/// Here is how a garden gnome sorts a line of flower pots:
///
/// Basically, he looks at the flower pot next to him and the previous one;
/// if they are in the right order he steps one pot forward, otherwise,
/// he swaps them and steps one pot backward.
///
/// Boundary conditions: if there is no previous pot, he steps forwards;
/// if there is no pot next to him, he is done.
///     
/// — "Gnome Sort - The Simplest Sort Algorithm". <https://dickgrune.com/Programs/gnomesort.html>
///
/// Complexity: O(n²)
pub fn gnome<const L: usize>(mut ar: [i32; L]) -> [i32; L] {
    let mut i = 0;

    while i < L {
        if i == 0 || ar[i] >= ar[i - 1] {
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

#[cfg(test)]
mod tests {
    use super::gnome;
    use crate::rand_array;

    #[test]
    fn test() {
        assert!(gnome(rand_array::<50>()).is_sorted())
    }
}
