#![feature(is_sorted)]

pub mod bubble;
pub mod gnome;
pub mod insertion;
pub mod prelude;
pub mod quick;
pub mod selection;

pub use prelude::*;
use rand::{thread_rng, Rng};

pub fn rand_array<const L: usize>() -> [i32; L] {
    let mut ar = [0; L];
    let mut rng = thread_rng();

    for i in 0..L {
        ar[i] = rng.gen();
    }

    ar
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::rand_array;

    #[test]
    fn test_random_array() {
        assert!(bubble(rand_array::<50>()).is_sorted())
    }
}
