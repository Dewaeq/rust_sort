mod bubble;
mod selection;

pub const LENGTH: usize = 5;
pub const INPUT: [i32; LENGTH] = [64, 25, 12, 22, 11];

fn main() {
    println!("Input:            {:?}", INPUT);
    println!("========================================");

    let mut output = selection::selection(INPUT, LENGTH);
    println!("Selection Sort:   {:?}", output);

    output = bubble::bubble(INPUT, LENGTH);
    println!("Bubble Sort:      {:?}", output);
}
