mod bubble;
mod selection;
mod insertion;
mod gnome;

pub const LENGTH: usize = 5;
pub const INPUT: [i32; LENGTH] = [64, 25, 12, 22, 11];

fn main() {
    println!("Input:                    {:?}", INPUT);
    println!("==============================================");

    let mut output = selection::selection(INPUT, LENGTH);
    println!("Selection Sort:           {:?}", output);

    output = bubble::bubble(INPUT, LENGTH);
    println!("Bubble Sort:              {:?}", output);

    output = bubble::bubble_recursive(INPUT, LENGTH);
    println!("Bubble Recursive Sort:    {:?}", output);
    
    output = insertion::insertion(INPUT, LENGTH);
    println!("Insertion Sort:           {:?}", output);
    
    output = insertion::insertion_recursive(INPUT, LENGTH);
    println!("Insertion Recursive Sort: {:?}", output);
    
    output = gnome::gnome(INPUT, LENGTH);
    println!("Gnome Sort:               {:?}", output);
}
