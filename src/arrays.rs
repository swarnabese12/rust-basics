//Arrays - Fixed list where elements are same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Reassign value
    numbers[2] = 120;

    //single value
    println!("Single val: {}", numbers[0]);

    //Array Length
    println!("Array length is: {}", numbers.len());

    println!("{:?}", numbers);

    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}
