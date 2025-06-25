pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of the string is transferred to `s2`
                 //println!("s1: {}",s1); // Error: `s1` is no longer valid
                 //here the ownership is transferred to s2 so s1 is no loner valid
    println!("s2: {}", s2);

    //Cloning to Retain Ownership
    //If you want to keep s1 valid, you need to clone the s1
    let p1 = String::from("hello");
    let p2 = p1.clone(); // Creates a deep copy
    println!("p1: {}, p2: {}", p1, p2);
}
