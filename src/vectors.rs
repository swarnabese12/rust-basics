//Vectors are resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Reassign value
    numbers[2] = 120;

    //Add on to vector
    numbers.push(6);
    numbers.push(7);

    println!("New Vector: {:?}",numbers);

    //pop vector
    numbers.pop();

    //single value
    println!("Single val: {}",numbers[0]);

    //Vector Length
    println!("Vector length is: {}",numbers.len());

    println!("{:?}",numbers);

    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}",slice);

    //Loop through vectors
    for x in numbers.iter(){
        println!("x: {}",x);
    }

    //Loop and Mutate
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers vector: {:?}",numbers);
}