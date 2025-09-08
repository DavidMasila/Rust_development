use std::io;

fn main() {
    //initialize an array for the user to select an item

    let a: [usize; 5] = [23, 45, 67, 89, 23];
    //instruct the user what to do

    println!("Hello, make a selection from 0 to 4!");
    //initialize a variable index that will be assigned to the keyboard input

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("You did not make a selection");
    //Using shadowing. Resassign index. Trim the user input and parse it to interger type usize

    let index: usize = index
        .trim()
        .parse()
        .expect("Ensure your selection is numeric!");

    //Create a varaiable to house the respective index item from the array
    let item_selected: usize = a[index];

    println!("You selected item is {item_selected}!!");
}
