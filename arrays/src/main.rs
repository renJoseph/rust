use std::mem;

fn main() {
    let mut arr: [i8; 10] = [1; 10];
    
    println!("{}\n", arr[arr.len() - 1]);
    
    arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for elem in arr {
        println!("{}", elem)
    }
    
    // Arrays are stack allocated
    println!("\narray occupies {} bytes\n", mem::size_of_val(&arr));
    
    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyse_slice(&arr);

        // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyse_slice(&arr[1 .. 4]);
}

fn analyse_slice(slice: &[i8]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}