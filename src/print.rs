pub fn run() {
    // print to console
    println!("Hello from print.rs file");

    //Basic Formatting
    println!("the number: {}",1);

    println!("{} is from {}","Swarna","Disney");

    //Positional Arguments
    println!("{0} is from {1} and {0} is {2}","Swarna","Disney","pretty");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "Swarna", activity = "CHESS");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",4,4,4);

    //Placeholder for debug trait
    println!("{:?}", ("lakshmi", 100, "abc", true));

    //Basic Math
    println!("SUM of numbers: {}", 20 + 45);
}